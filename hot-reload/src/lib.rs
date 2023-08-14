mod lib_set;
mod types;
use std::fs::metadata;

use std::time::Duration;

use bevy::a11y::{AccessibilityRequested, Focus};

use bevy::ecs::prelude::*;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::PreUpdate;
use bevy::utils::Instant;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;
use bevy::MinimalPlugins;

pub extern crate libloading;
pub extern crate reload_macros;

use lib_set::*;
use libloading::Library;
pub use types::*;

/// Only for HotReload internal use. Must be pub because it is
/// inserted as an arg on systems with #[make_hot]
#[derive(Resource)]
struct HotReloadLibInternalUseOnly {
    pub library: Option<Library>,
    pub updated_this_frame: bool,
    pub last_update_time: Instant,
    pub cargo_watch_child: ChildGuard,
    pub library_paths: LibPathSet,
}

fn update_lib(
    mut hot_reload_int: ResMut<HotReloadLibInternalUseOnly>,
    mut hot_reload: ResMut<HotReload>,
    mut event: EventWriter<HotReloadEvent>,
) {
    hot_reload_int.updated_this_frame = false;
    hot_reload.updated_this_frame = false;

    let lib_file_path = hot_reload_int.library_paths.lib_file_path();
    let hot_in_use_file_path = hot_reload_int.library_paths.hot_in_use_file_path();

    // copy over and load lib if it has been updated, or hasn't been initially
    if lib_file_path.is_file() {
        bevy::log::info!("Found Lib");
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

pub fn run_reloadabe_app(options: Option<HotReloadOptions>) {
    let library_paths = LibPathSet::new(options.unwrap_or_default()).unwrap();
    println!("Paths: {library_paths:?}");

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

    let mut app = bevy::app::App::new();

    app.init_resource::<AccessibilityRequested>()
        .init_resource::<Focus>()
        .init_resource::<HotReload>()
        .add_plugins((
            MinimalPlugins,
            WindowPlugin::default(),
            InputPlugin,
            WinitPlugin,
            LogPlugin::default(),
        ))
        .add_event::<HotReloadEvent>()
        .insert_resource(HotReloadLibInternalUseOnly {
            cargo_watch_child: child,
            library: None,
            updated_this_frame: false,
            // Using 1 second ago so to trigger lib load immediately instead of in 1 second
            last_update_time: Instant::now().checked_sub(Duration::from_secs(1)).unwrap(),
            library_paths,
        })
        .add_systems(PreUpdate, update_lib);

    app.run()
}
