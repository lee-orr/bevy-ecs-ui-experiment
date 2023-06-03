pub mod components;
pub mod style;
pub mod ui_bundle_spawner;
pub mod ui_id;

use bevy::prelude::*;

pub use components::*;
pub use style::*;
pub use ui_bundle_spawner::*;
pub use ui_id::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, _app: &mut App) {}
}
