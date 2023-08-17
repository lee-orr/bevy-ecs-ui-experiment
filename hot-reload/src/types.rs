use std::path::PathBuf;

use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::*,
    utils::{HashMap, HashSet, Instant},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::InternalHotReload;

#[derive(Resource, Default, Reflect)]
pub struct HotReload {
    pub last_updated_frame: usize,
    pub version: usize,
    pub updated_this_frame: bool,
}

#[derive(Debug, Event, Reflect)]
pub struct HotReloadEvent {
    pub last_update_time: Instant,
}

#[derive(Debug, Default, Reflect)]
pub struct HotReloadOptions {
    pub lib_name: Option<String>,
    pub watch_folder: Option<PathBuf>,
    pub target_folder: Option<PathBuf>,
}

#[derive(Default, Resource, Clone, Debug)]
pub struct ReloadableAppCleanup {
    pub labels: HashSet<Box<dyn ScheduleLabel>>,
}

#[derive(Default, Resource)]
pub struct ReloadableAppContents {
    schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
    resources: HashSet<String>,
    components: HashSet<String>,
}

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ReloadableSchedule<T: ScheduleLabel>(T);

impl<T: ScheduleLabel> ReloadableSchedule<T> {
    pub fn get_inner(&self) -> &T {
        &self.0
    }
}

impl<T: ScheduleLabel> ReloadableSchedule<T> {
    pub fn new(label: T) -> Self {
        Self(label)
    }
}

mod private {
    pub trait ReloadableAppSealed {}
}

pub trait ReloadableApp: private::ReloadableAppSealed {
    fn add_systems<M, L: ScheduleLabel + Eq + ::std::hash::Hash + Clone>(
        &mut self,
        schedule: L,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;

    fn insert_reloadable_resource<R: ReloadableResource>(&mut self) -> &mut Self;
    fn reset_resource<R: Resource + Default>(&mut self) -> &mut Self;
    fn reset_resource_to_value<R: Resource + Clone>(&mut self, value: R) -> &mut Self;
    fn register_reloadable_component<C: ReloadableComponent>(&mut self) -> &mut Self;
    fn clear_marked_on_reload<C: Component>(&mut self) -> &mut Self;
    fn reset_setup<C: Component, M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self;
    fn reset_setup_in_state<C: Component, S: States, M>(
        &mut self,
        state: S,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl ReloadableAppContents {
    pub(crate) fn schedule_iter(self) -> impl Iterator<Item = (Box<dyn ScheduleLabel>, Schedule)> {
        self.schedules.into_iter()
    }
}

impl private::ReloadableAppSealed for ReloadableAppContents {}

impl ReloadableApp for ReloadableAppContents {
    fn add_systems<M, L: ScheduleLabel + Eq + ::std::hash::Hash + Clone>(
        &mut self,
        schedule: L,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let schedules = &mut self.schedules;
        let schedule: Box<dyn ScheduleLabel> = Box::new(schedule);

        if let Some(schedule) = schedules.get_mut(&schedule) {
            println!("Adding systems to schedule");
            schedule.add_systems(systems);
        } else {
            println!("Creating schedule with systems");
            let mut new_schedule = Schedule::new();
            new_schedule.add_systems(systems);
            schedules.insert(schedule, new_schedule);
        }

        self
    }

    fn insert_reloadable_resource<R: ReloadableResource>(&mut self) -> &mut Self {
        let name = R::get_type_name();
        if !self.resources.contains(name) {
            self.resources.insert(name.to_string());
            println!("adding resource {name}");
            self.add_systems(SerializeReloadables, serialize_reloadable_resource::<R>)
                .add_systems(DeserializeReloadables, deserialize_reloadable_resource::<R>);
        }
        self
    }

    fn register_reloadable_component<C: ReloadableComponent>(&mut self) -> &mut Self {
        let name = C::get_type_name();
        if !self.components.contains(name) {
            self.components.insert(name.to_string());
            self.add_systems(SerializeReloadables, serialize_reloadable_component::<C>)
                .add_systems(
                    DeserializeReloadables,
                    deserialize_reloadable_component::<C>,
                );
        }
        self
    }

    fn reset_resource<R: Resource + Default>(&mut self) -> &mut Self {
        println!("resetting resource");
        self.add_systems(DeserializeReloadables, |mut commands: Commands| {
            commands.insert_resource(R::default());
        });
        self
    }

    fn reset_resource_to_value<R: Resource + Clone>(&mut self, value: R) -> &mut Self {
        println!("resetting resource");
        self.add_systems(DeserializeReloadables, move |mut commands: Commands| {
            commands.insert_resource(value.clone());
        });
        self
    }

    fn clear_marked_on_reload<C: Component>(&mut self) -> &mut Self {
        self.add_systems(CleanupReloaded, clear_marked_system::<C>);
        self
    }

    fn reset_setup<C: Component, M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        self.add_systems(CleanupReloaded, clear_marked_system::<C>)
            .add_systems(OnReloadComplete, systems)
    }

    fn reset_setup_in_state<C: Component, S: States, M>(
        &mut self,
        state: S,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_systems(CleanupReloaded, clear_marked_system::<C>)
            .add_systems(OnExit(state.clone()), clear_marked_system::<C>)
            .add_systems(
                PreUpdate,
                systems.run_if(
                    in_state(state)
                        .and_then(hot_reload_occured.or_else(resource_changed::<State<S>>())),
                ),
            )
    }
}

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SerializeReloadables;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeserializeReloadables;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SetupReload;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CleanupReloaded;
#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct OnReloadComplete;

pub trait ReloadableSetup {
    fn setup_function_name() -> &'static str;
    fn default_function(app: &mut ReloadableAppContents);
}

pub trait ReloadableResource: Resource + Serialize + DeserializeOwned + Default {
    fn get_type_name() -> &'static str;
}

pub trait ReloadableComponent: Component + Serialize + DeserializeOwned + Default {
    fn get_type_name() -> &'static str;
}

#[derive(Resource, Default)]
pub struct ReloadableResourceStore {
    map: HashMap<String, Vec<u8>>,
}

fn serialize_reloadable_resource<R: ReloadableResource>(
    mut store: ResMut<ReloadableResourceStore>,
    resource: Option<Res<R>>,
    mut commands: Commands,
) {
    let Some(resource) = resource else {
        return;
    };
    if let Ok(v) = rmp_serde::to_vec(resource.as_ref()) {
        store.map.insert(R::get_type_name().to_string(), v);
    }

    commands.remove_resource::<R>();
}

fn deserialize_reloadable_resource<R: ReloadableResource>(
    store: Res<ReloadableResourceStore>,
    mut commands: Commands,
) {
    let name = R::get_type_name();
    println!("Deserializing {name}");
    let v: R = store
        .map
        .get(name)
        .and_then(|v| rmp_serde::from_slice(v.as_slice()).ok())
        .unwrap_or_default();

    commands.insert_resource(v);
}

#[derive(Resource, Default)]
pub struct ReloadableComponentStore {
    map: HashMap<String, Vec<(Entity, Vec<u8>)>>,
}

fn serialize_reloadable_component<C: ReloadableComponent>(
    mut store: ResMut<ReloadableComponentStore>,
    query: Query<(Entity, &C)>,
    mut commands: Commands,
) {
    let name = C::get_type_name();
    for (entity, component) in query.iter() {
        if let Ok(v) = rmp_serde::to_vec(component) {
            let storage = store.map.entry(name.to_string()).or_default();
            storage.push((entity, v));
        }

        commands.entity(entity).remove::<C>();
    }
}

fn deserialize_reloadable_component<C: ReloadableComponent>(
    mut store: ResMut<ReloadableComponentStore>,
    mut commands: Commands,
) {
    let name = C::get_type_name();
    println!("Deserializing {name}");

    if let Some(storage) = store.map.remove(name) {
        for (entity, value) in storage.into_iter() {
            let v: C = rmp_serde::from_slice(&value).ok().unwrap_or_default();
            commands.entity(entity).insert(v);
        }
    }
}

pub fn clear_marked_system<C: Component>(mut commands: Commands, q: Query<Entity, With<C>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn hot_reload_occured(reload: Res<InternalHotReload>) -> bool {
    reload.updated_this_frame
}
