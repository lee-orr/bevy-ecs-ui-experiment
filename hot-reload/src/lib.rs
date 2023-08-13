mod types;
use bevy::a11y::{AccessibilityRequested, Focus};
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::ecs::prelude::*;
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;
use bevy::{DefaultPlugins, MinimalPlugins};

pub extern crate reload_macros;

pub use types::ReloadableApp;

#[derive(Resource)]
pub struct HotReload {
    pub last_updated_frame: usize,
    pub version: usize,
}

pub trait HotReloadablePlugins {
    fn remove_winit(self) -> PluginGroupBuilder;
}

impl<T: PluginGroup> HotReloadablePlugins for T {
    fn remove_winit(self) -> PluginGroupBuilder {
        self.build().disable::<bevy::winit::WinitPlugin>()
    }
}

pub fn run_reloadabe_app() {
    let mut app = bevy::app::App::new();

    app.init_resource::<AccessibilityRequested>()
        .init_resource::<Focus>()
        .add_plugins((
            MinimalPlugins,
            WindowPlugin::default(),
            InputPlugin,
            WinitPlugin,
        ));

    app.run()
}
