use std::path::PathBuf;

use bevy::{
    app::{AppLabel, PluginGroupBuilder},
    prelude::{Event, PluginGroup, Resource},
    utils::Instant,
};
pub trait ReloadableComponent {}

pub trait ReloadableResource {}

#[derive(AppLabel)]
pub struct ReloadableApp;

#[derive(Resource, Default)]
pub struct HotReload {
    pub last_updated_frame: usize,
    pub version: usize,
    pub updated_this_frame: bool,
}

#[derive(Debug, Event)]
pub struct HotReloadEvent {
    pub last_update_time: Instant,
}

pub trait HotReloadablePlugins {
    fn setup_for_hot_reload(self) -> PluginGroupBuilder;
}

impl<T: PluginGroup> HotReloadablePlugins for T {
    fn setup_for_hot_reload(self) -> PluginGroupBuilder {
        #[cfg(not(feature = "bypass"))]
        {
            self.build().disable::<bevy::winit::WinitPlugin>()
        }
        #[cfg(feature = "bypass")]
        {
            self.build()
        }
    }
}

#[derive(Debug, Default)]
pub struct HotReloadOptions {
    pub lib_name: Option<String>,
    pub watch_folder: Option<PathBuf>,
    pub target_folder: Option<PathBuf>,
}
