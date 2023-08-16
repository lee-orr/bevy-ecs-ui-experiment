mod lib_set;
mod types;
use std::fs::metadata;

use std::time::Duration;

use bevy::ecs::prelude::*;

use bevy::prelude::{App, Plugin, PreUpdate};

use bevy::utils::Instant;

pub extern crate libloading;
pub extern crate reload_macros;

use lib_set::*;
use libloading::Library;
pub use types::*;

#[derive(Resource)]
struct InternalHotReload {
    pub library: Option<Library>,
    pub updated_this_frame: bool,
    pub last_update_time: Instant,
    pub library_paths: LibPathSet,

    #[allow(dead_code)]
    pub cargo_watch_child: ChildGuard,
}

fn update_lib(
    mut hot_reload_int: ResMut<InternalHotReload>,
    mut hot_reload: ResMut<HotReload>,
    mut event: EventWriter<HotReloadEvent>,
) {
    hot_reload_int.updated_this_frame = false;
    hot_reload.updated_this_frame = false;

    let lib_file_path = hot_reload_int.library_paths.lib_file_path();
    let hot_in_use_file_path = hot_reload_int.library_paths.hot_in_use_file_path();

    // copy over and load lib if it has been updated, or hasn't been initially
    if lib_file_path.is_file() {
        if hot_in_use_file_path.is_file() {
            let hot_lib_meta = metadata(&hot_in_use_file_path).unwrap();
            let main_lib_meta = metadata(&lib_file_path).unwrap();
            if hot_lib_meta.modified().unwrap() < main_lib_meta.modified().unwrap()
                && hot_reload_int.last_update_time.elapsed() > Duration::from_secs(1)
            {
                hot_reload_int.library = None;
                let _ = std::fs::copy(lib_file_path, &hot_in_use_file_path);
            }
        } else {
            hot_reload_int.library = None;
            std::fs::copy(lib_file_path, &hot_in_use_file_path).unwrap();
        }
        if hot_reload_int.library.is_none() {
            bevy::log::info!("No library set");
            unsafe {
                let lib = libloading::Library::new(&hot_in_use_file_path).unwrap_or_else(|_| {
                    panic!(
                        "Can't open required library {}",
                        &hot_in_use_file_path.to_string_lossy()
                    )
                });

                hot_reload_int.library = Some(lib);
                hot_reload_int.updated_this_frame = true;
                hot_reload_int.last_update_time = Instant::now();
                bevy::log::info!("Hot Reloaded");
                event.send(HotReloadEvent {
                    last_update_time: hot_reload_int.last_update_time,
                });
            }
        }
    }

    hot_reload.updated_this_frame = hot_reload_int.updated_this_frame;
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

#[derive(Default)]
pub struct HotReloadPlugin(HotReloadOptions);

impl HotReloadPlugin {
    pub fn new(options: HotReloadOptions) -> Self {
        Self(options)
    }
}

impl Plugin for HotReloadPlugin {
    fn build(&self, app: &mut App) {
        let options = &self.0;
        let library_paths = LibPathSet::new(options).unwrap();
        println!("Paths: {library_paths:?}");

        let _ = std::fs::remove_file(library_paths.lib_file_path());

        let build_cmd = format!(
            "build --lib --target-dir {} --features bevy/dynamic_linking",
            library_paths.folder.parent().unwrap().to_string_lossy()
        );

        let child = ChildGuard({
            let mut cmd = std::process::Command::new("cargo");

            cmd.arg("watch")
                .arg("--watch-when-idle")
                .arg("-w")
                .arg(library_paths.watch_folder.as_os_str())
                .arg("-x")
                .arg(build_cmd);
            println!("Spawning command: {cmd:?}");

            cmd.spawn()
                .expect("cargo watch command failed, make sure cargo watch is installed")
        });

        let reload_schedule = Schedule::new();
        let cleanup_schedule = Schedule::new();

        app.add_schedule(SetupReload, reload_schedule)
            .add_schedule(CleanupReloaded, cleanup_schedule)
            .register_type::<HotReload>()
            .register_type::<HotReloadEvent>()
            .init_resource::<HotReload>()
            .init_resource::<ReloadableAppInner>()
            .init_non_send_resource::<ReloadedApp>()
            .add_event::<HotReloadEvent>()
            .insert_resource(InternalHotReload {
                cargo_watch_child: child,
                library: None,
                updated_this_frame: false,
                // Using 1 second ago so to trigger lib load immediately instead of in 1 second
                last_update_time: Instant::now().checked_sub(Duration::from_secs(1)).unwrap(),
                library_paths,
            })
            .add_systems(PreUpdate, (update_lib, cleanup, reload).chain());
    }
}

#[derive(Default)]
struct ReloadedApp(Option<Box<App>>);

pub trait ReloadableAppSetup {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self;
}

impl ReloadableAppSetup for App {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self {
        let name = T::setup_function_name().as_bytes();
        let system = move |world: &mut World| setup_reloadable_app(name, world);
        self.add_systems(SetupReload, system)
    }
}

fn setup_reloadable_app(name: &'static [u8], world: &mut World) {
    let Some(internal_state) = world.remove_resource::<InternalHotReload>() else {
        return;
    };
    if !internal_state.updated_this_frame {
        world.insert_resource(internal_state);
        return;
    }
    let Some(lib) = &internal_state.library else {
        world.insert_resource(internal_state);
        return;
    };

    let mut reloadable = ReloadableApp::new(world);
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn(&mut ReloadableApp)> = lib
            .get(name)
            .unwrap_or_else(|_| panic!("Can't find reloadable setup function",));
        func(&mut reloadable)
    };
    world.insert_resource(internal_state);
}

fn cleanup(mut schedules: ResMut<Schedules>, inner: Res<ReloadableAppInner>) {
    for schedule in inner.labels.iter() {
        let updated = Schedule::new();
        schedules.insert(schedule.dyn_clone(), updated);
    }
}

fn reload(world: &mut World) {
    let internal_state = world.resource::<InternalHotReload>();
    if !internal_state.updated_this_frame {
        return;
    }
    let Some(_lib) = &internal_state.library else {
        return;
    };
    let _ = world.try_run_schedule(SetupReload);
}
