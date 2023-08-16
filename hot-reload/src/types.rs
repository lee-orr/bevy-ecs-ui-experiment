use std::path::PathBuf;

use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::*,
    utils::{HashSet, Instant},
};
use libloading::Library;

pub trait ReloadableComponent {}

pub trait ReloadableResource {}

pub struct RunFrame;

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
pub(crate) struct ReloadableAppInner {
    pub labels: HashSet<Box<dyn ScheduleLabel>>,
}

pub struct ReloadableApp<'w> {
    inner: ReloadableAppInner,
    world: &'w mut World,
}

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash)]
pub struct ReloadableSchedule<T: ScheduleLabel + Eq + ::std::hash::Hash + Clone>(T);

impl<T: ScheduleLabel + Eq + ::std::hash::Hash + Clone> Clone for ReloadableSchedule<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ScheduleLabel + Eq + ::std::hash::Hash + Clone> ReloadableSchedule<T> {
    pub fn new(label: &T) -> Self {
        Self(label.clone())
    }
}

impl<'w> ReloadableApp<'w> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        let inner = world
            .get_resource::<ReloadableAppInner>()
            .cloned()
            .unwrap_or_default();
        Self { inner, world }
    }

    pub fn add_systems<M, L: ScheduleLabel + Eq + ::std::hash::Hash + Clone>(
        &mut self,
        original: L,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let schedule = ReloadableSchedule::new(&original);

        if self.inner.labels.insert(schedule.dyn_clone()) {
            println!("Schedule {schedule:?} added");
            let Some(mut schedules) = self.world.get_resource_mut::<Schedules>() else {
                return self;
            };
            let target = schedule.clone();
            let runner = move |world: &mut World| run_reloadable_schedule(&target, world);

            if let Some(schedule) = schedules.get_mut(&original) {
                schedule.add_systems(runner);
            } else {
                let mut new_schedule = Schedule::new();
                new_schedule.add_systems(runner);
                schedules.insert(original, new_schedule);
            }
        } else {
            println!("Schedule {schedule:?} already existed");
        }

        let mut schedules = self.world.resource_mut::<Schedules>();

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
}

fn run_reloadable_schedule<T: AsRef<dyn ScheduleLabel>>(label: &T, world: &mut World) {
    let _ = world.try_run_schedule(label);
}

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SetupReload;

#[derive(ScheduleLabel, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CleanupReloaded;

pub trait ReloadableSetup {
    fn setup_function_name() -> &'static str;
    fn default_function(app: &mut ReloadableApp);
}

pub struct LibraryHolder(Option<Library>, PathBuf);

impl Drop for LibraryHolder {
    fn drop(&mut self) {
        self.0 = None;
        std::fs::remove_file(&self.1);
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
