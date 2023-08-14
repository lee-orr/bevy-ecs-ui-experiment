use std::{path::PathBuf, sync::mpsc};

use bevy::{
    app::PluginGroupBuilder,
    ecs::system::SystemParam,
    input::keyboard::*,
    input::mouse::*,
    input::{touchpad::*, *},
    prelude::*,
    utils::Instant,
    window::*,
};

pub trait ReloadableComponent {}

pub trait ReloadableResource {}

pub struct RunFrame;

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

pub fn app_grabber() -> (oneshot::Sender<App>, oneshot::Receiver<App>) {
    oneshot::channel()
}

#[derive(SystemParam)]
struct WindowAndInputEventWriters<'w> {
    // `winit` `WindowEvent`s
    window_resized: EventWriter<'w, WindowResized>,
    window_close_requested: EventWriter<'w, WindowCloseRequested>,
    window_scale_factor_changed: EventWriter<'w, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventWriter<'w, WindowBackendScaleFactorChanged>,
    window_focused: EventWriter<'w, WindowFocused>,
    window_moved: EventWriter<'w, WindowMoved>,
    window_theme_changed: EventWriter<'w, WindowThemeChanged>,
    window_destroyed: EventWriter<'w, WindowDestroyed>,
    keyboard_input: EventWriter<'w, KeyboardInput>,
    character_input: EventWriter<'w, ReceivedCharacter>,
    mouse_button_input: EventWriter<'w, MouseButtonInput>,
    touchpad_magnify_input: EventWriter<'w, TouchpadMagnify>,
    touchpad_rotate_input: EventWriter<'w, TouchpadRotate>,
    mouse_wheel_input: EventWriter<'w, MouseWheel>,
    touch_input: EventWriter<'w, TouchInput>,
    ime_input: EventWriter<'w, Ime>,
    file_drag_and_drop: EventWriter<'w, FileDragAndDrop>,
    cursor_moved: EventWriter<'w, CursorMoved>,
    cursor_entered: EventWriter<'w, CursorEntered>,
    cursor_left: EventWriter<'w, CursorLeft>,
    // `winit` `DeviceEvent`s
    mouse_motion: EventWriter<'w, MouseMotion>,
}

#[derive(SystemParam)]
struct WindowAndInputEventReaders<'w, 's> {
    // `winit` `WindowEvent`s
    window_resized: EventReader<'w, 's, WindowResized>,
    window_close_requested: EventReader<'w, 's, WindowCloseRequested>,
    window_scale_factor_changed: EventReader<'w, 's, WindowScaleFactorChanged>,
    window_backend_scale_factor_changed: EventReader<'w, 's, WindowBackendScaleFactorChanged>,
    window_focused: EventReader<'w, 's, WindowFocused>,
    window_moved: EventReader<'w, 's, WindowMoved>,
    window_theme_changed: EventReader<'w, 's, WindowThemeChanged>,
    window_destroyed: EventReader<'w, 's, WindowDestroyed>,
    keyboard_input: EventReader<'w, 's, KeyboardInput>,
    character_input: EventReader<'w, 's, ReceivedCharacter>,
    mouse_button_input: EventReader<'w, 's, MouseButtonInput>,
    touchpad_magnify_input: EventReader<'w, 's, TouchpadMagnify>,
    touchpad_rotate_input: EventReader<'w, 's, TouchpadRotate>,
    mouse_wheel_input: EventReader<'w, 's, MouseWheel>,
    touch_input: EventReader<'w, 's, TouchInput>,
    ime_input: EventReader<'w, 's, Ime>,
    file_drag_and_drop: EventReader<'w, 's, FileDragAndDrop>,
    cursor_moved: EventReader<'w, 's, CursorMoved>,
    cursor_entered: EventReader<'w, 's, CursorEntered>,
    cursor_left: EventReader<'w, 's, CursorLeft>,
    // `winit` `DeviceEvent`s
    mouse_motion: EventReader<'w, 's, MouseMotion>,
}
