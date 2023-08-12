pub mod components;
pub mod style;
pub mod ui_bundle_spawner;
pub mod ui_id;
mod ui_schedule;

use bevy::prelude::*;

pub use components::*;
pub use style::*;
pub use ui_bundle_spawner::*;
pub use ui_id::*;

pub use ui_schedule::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {}
}
