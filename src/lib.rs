pub mod base_components;
pub mod style_structs;
pub mod ui_bundle_spawner;
pub mod ui_id;

use bevy::prelude::*;

pub use base_components::*;
pub use style_structs::*;
pub use ui_bundle_spawner::*;
pub use ui_id::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, _app: &mut App) {}
}
