use std::path::PathBuf;

use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::*,
    reflect,
    utils::{HashMap, HashSet, Instant},
};
use libloading::Library;
use serde::{de::DeserializeOwned, Serialize};

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
pub(crate) struct ReloadableAppCleanup {
    pub labels: HashSet<Box<dyn ScheduleLabel>>,
}

#[derive(Default, Resource)]
pub struct ReloadableAppContents {
    schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
    resources: HashSet<String>,
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
    store: ResMut<ReloadableResourceStore>,
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

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SerializeReloadables;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeserializeReloadables;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SetupReload;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CleanupReloaded;

pub trait ReloadableSetup {
    fn setup_function_name() -> &'static str;
    fn default_function(app: &mut ReloadableAppContents);
}

pub struct LibraryHolder(Option<Library>, PathBuf);

impl Drop for LibraryHolder {
    fn drop(&mut self) {
        self.0 = None;
        let _ = std::fs::remove_file(&self.1);
    }
}

impl LibraryHolder {
    pub fn new(path: &PathBuf) -> Option<Self> {
        let extension = path.extension();
        let uuid = uuid::Uuid::new_v4();
        let new_path = path.clone();
        let mut new_path = new_path.with_file_name(uuid.to_string());
        if let Some(extension) = extension {
            new_path.set_extension(extension);
        }
        println!("New path: {new_path:?}");
        std::fs::rename(path, &new_path).ok()?;
        println!("Copied file to new path");

        let lib = unsafe { libloading::Library::new(&new_path).ok() }?;
        println!("Loaded library");
        Some(Self(Some(lib), new_path))
    }
    pub fn library(&self) -> Option<&Library> {
        self.0.as_ref()
    }
}

pub trait ReloadableResource: Resource + Serialize + DeserializeOwned + Default {
    fn get_type_name() -> &'static str;
}
