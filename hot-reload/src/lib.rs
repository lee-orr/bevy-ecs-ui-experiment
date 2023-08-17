mod lib_set;
mod types;

use std::thread;

use bevy_hot_winit::HotWinitPlugin;

use std::sync::Arc;
use std::time::Duration;

use bevy::ecs::prelude::*;

use bevy::prelude::{App, First, Plugin, PreStartup, PreUpdate};

use bevy::utils::Instant;

pub extern crate libloading;
pub extern crate reload_macros;

use lib_set::*;
pub use types::*;

#[derive(Resource)]
struct InternalHotReload {
    pub library: Option<Arc<LibraryHolder>>,
    pub last_lib: Option<Arc<LibraryHolder>>,
    pub updated_this_frame: bool,
    pub last_update_time: Instant,
    pub libs: LibPathSet,
}

fn update_lib(library_paths: &LibPathSet) -> Option<Arc<LibraryHolder>> {
    let lib_file_path = library_paths.lib_file_path();

    if lib_file_path.is_file() {
        println!("Found library {lib_file_path:?}");
        let Some(holder) = LibraryHolder::new(&lib_file_path) else {
            return None;
        };
        println!("Generated file holder");
        Some(Arc::new(holder))
    } else {
        None
    }
}

fn update_lib_system(
    mut hot_reload_int: ResMut<InternalHotReload>,
    mut hot_reload: ResMut<HotReload>,
    mut event: EventWriter<HotReloadEvent>,
) {
    hot_reload_int.updated_this_frame = false;
    hot_reload.updated_this_frame = false;

    if let Some(lib) = update_lib(&hot_reload_int.libs) {
        println!("Got Update");
        hot_reload_int.last_lib = hot_reload_int.library.clone();
        hot_reload_int.library = Some(lib);
        hot_reload_int.updated_this_frame = true;
        hot_reload.updated_this_frame = true;
        hot_reload_int.last_update_time = Instant::now();
    }

    hot_reload.updated_this_frame = hot_reload_int.updated_this_frame;
    event.send(HotReloadEvent {
        last_update_time: hot_reload_int.last_update_time,
    });
}

pub struct ChildGuard(pub std::process::Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.0.kill() {
            Err(e) => println!("Could not kill cargo watch process: {}", e),
            Ok(_) => println!("Successfully killed cargo watch process"),
        }
    }
}

pub enum ReloadLibEvent {
    UpdatedLib(Arc<LibraryHolder>),
    Error(String),
}

pub struct EndWatch;

pub fn run_reloadabe_app(options: HotReloadOptions) {
    println!("Current Thread: {:?}", std::thread::current().id());
    let library_paths = LibPathSet::new(&options).unwrap();
    println!("Paths: {library_paths:?}");

    let _ = std::fs::remove_file(library_paths.lib_file_path());

    let (end_watch_tx, end_watch_rx) = crossbeam::channel::bounded::<EndWatch>(1);
    let (_reload_lib_tx, _reload_lib_rx) = crossbeam::channel::bounded::<ReloadLibEvent>(5);

    let end_cargo_watch_rx = end_watch_rx.clone();
    let watch_folder = library_paths.watch_folder.clone();
    let folder = library_paths.folder.clone();

    thread::spawn(move || {
        println!("Spawned watch thread");
        println!("Watch Thread: {:?}", std::thread::current().id());
        let build_cmd = format!(
            "build --lib --target-dir {} --features bevy/dynamic_linking",
            folder.parent().unwrap().to_string_lossy()
        );

        let mut cmd = std::process::Command::new("cargo");

        cmd.arg("watch")
            .arg("--watch-when-idle")
            .arg("-w")
            .arg(watch_folder.as_os_str())
            .arg("-x")
            .arg(build_cmd);
        println!("Spawning command: {cmd:?}");

        let h = cmd
            .spawn()
            .expect("cargo watch command failed, make sure cargo watch is installed");
        println!("spawned watcher");

        let _ = end_cargo_watch_rx.recv();

        println!("Closing out {:?}", h);
    });

    let lib = get_initial_library(&library_paths);

    if let Some(lib) = lib.library() {
        println!("Executing first run");
        unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn(HotReloadPlugin)> = lib
                .get("hot_reload_internal_main".as_bytes())
                .unwrap_or_else(|_| panic!("Can't find main function",));
            println!("Run App Thread: {:?}", std::thread::current().id());
            func(HotReloadPlugin::new(library_paths.clone()));
        };
    } else {
        eprint!("Library still somehow missing");
    }
    println!("Got to the end for some reason...");

    let _ = end_watch_tx.send(EndWatch);
}

fn get_initial_library(library_paths: &LibPathSet) -> Arc<LibraryHolder> {
    loop {
        if let Some(library) = update_lib(library_paths) {
            println!("Update Thread: {:?}", std::thread::current().id());
            println!("Updated lib");
            return library;
        }
    }
}

pub struct HotReloadPlugin(LibPathSet);

impl HotReloadPlugin {
    pub fn new(libs: LibPathSet) -> Self {
        Self(libs)
    }
}

impl Plugin for HotReloadPlugin {
    fn build(&self, app: &mut App) {
        println!(
            "Build Hot Reload Plugin Thread: {:?}",
            std::thread::current().id()
        );
        let reload_schedule = Schedule::new();
        let cleanup_schedule = Schedule::new();
        let serialize_schedule = Schedule::new();
        let deserialize_schedule = Schedule::new();
        let reload_complete = Schedule::new();

        app.add_plugins(HotWinitPlugin)
            .add_schedule(SetupReload, reload_schedule)
            .add_schedule(CleanupReloaded, cleanup_schedule)
            .add_schedule(SerializeReloadables, serialize_schedule)
            .add_schedule(DeserializeReloadables, deserialize_schedule)
            .add_schedule(OnReloadComplete, reload_complete)
            .init_resource::<HotReload>()
            .init_resource::<ReloadableAppContents>()
            .init_resource::<ReloadableAppCleanup>()
            .init_resource::<ReloadableResourceStore>()
            .init_resource::<ReloadableComponentStore>()
            .add_event::<HotReloadEvent>()
            .insert_resource(InternalHotReload {
                library: None,
                last_lib: None,
                updated_this_frame: true,
                last_update_time: Instant::now().checked_sub(Duration::from_secs(1)).unwrap(),
                libs: self.0.clone(),
            })
            .add_systems(PreStartup, reload)
            .add_systems(CleanupReloaded, cleanup)
            .add_systems(First, (update_lib_system, reload).chain());
        println!("Finished build");
    }
}

pub trait ReloadableAppSetup {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self;
}

impl ReloadableAppSetup for App {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self {
        let name = T::setup_function_name();
        let system = move |world: &mut World| setup_reloadable_app::<T>(name, world);
        self.add_systems(SetupReload, system)
    }
}

fn setup_reloadable_app<T: ReloadableSetup>(name: &'static str, world: &mut World) {
    println!("Setting up reloadables at {name}");
    let Some(internal_state) = world.get_resource::<InternalHotReload>() else {
        return;
    };

    println!("got internal reload state");

    let Some(lib) = &internal_state.library else {
        println!("can't get library");
        let Some(mut reloadable) = world.get_resource_mut::<ReloadableAppContents>() else {
            return;
        };
        println!("setup default");

        T::default_function(&mut reloadable);
        return;
    };
    let lib = lib.clone();
    let Some(lib) = lib.library() else {
        println!("can't access library internals ");
        let Some(mut reloadable) = world.get_resource_mut::<ReloadableAppContents>() else {
            return;
        };
        println!("setup default");
        T::default_function(&mut reloadable);
        return;
    };

    let Some(mut reloadable) = world.get_resource_mut::<ReloadableAppContents>() else {
        println!("no reloadable app");
        return;
    };
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(&mut ReloadableAppContents)> = lib
            .get(name.as_bytes())
            .unwrap_or_else(|_| panic!("Can't find reloadable setup function",));
        func(&mut reloadable)
    };
    println!("setup for {name} complete");
}

fn register_schedules(world: &mut World) {
    println!("Reloading schedules");
    let Some(reloadable) = world.remove_resource::<ReloadableAppContents>() else {
        return;
    };
    println!("Has reloadable app");

    let Some(mut schedules) = world.get_resource_mut::<Schedules>() else {
        return;
    };

    println!("Has schedules resource");

    let mut inner = ReloadableAppCleanup::default();

    for (original, schedule) in reloadable.schedule_iter() {
        let label = ReloadableSchedule::new(original.clone());
        println!("Adding {label:?} to schedule");
        inner.labels.insert(Box::new(label.clone()));
        let exists = schedules.insert(label.clone(), schedule);
        if exists.is_none() {
            if let Some(root) = schedules.get_mut(&original) {
                let label = label.clone();
                root.add_systems(move |w: &mut World| {
                    let _ = w.try_run_schedule(label.clone());
                });
            } else {
                let label = label.clone();
                let mut root = Schedule::new();
                root.add_systems(move |w: &mut World| {
                    let _ = w.try_run_schedule(label.clone());
                });
                schedules.insert(original, root);
            }
        }
    }

    world.insert_resource(inner);
}

fn cleanup(
    mut commands: Commands,
    mut schedules: ResMut<Schedules>,
    reloadable: Res<ReloadableAppCleanup>,
) {
    for schedule in reloadable.labels.iter() {
        println!("Attempting cleanup for {schedule:?}");
        let cleadn = schedules.insert(schedule.clone(), Schedule::default());
        println!(
            "Tried cleaning {schedule:?} was empty: {}",
            cleadn.is_none()
        );
    }
    println!("Cleanup almost complete");

    commands.insert_resource(ReloadableAppContents::default());
    println!("Cleanup complete");
}

fn reload(world: &mut World) {
    let internal_state = world.resource::<InternalHotReload>();
    if !internal_state.updated_this_frame {
        return;
    }
    println!("Serializing...");
    let _ = world.try_run_schedule(SerializeReloadables);
    println!("Cleanup...");
    let _ = world.try_run_schedule(CleanupReloaded);
    println!("Setup...");
    let _ = world.try_run_schedule(SetupReload);
    println!("Set Schedules...");
    register_schedules(world);
    println!("Deserialize...");
    let _ = world.try_run_schedule(DeserializeReloadables);
    println!("reload complete");
    let _ = world.try_run_schedule(OnReloadComplete);
}
