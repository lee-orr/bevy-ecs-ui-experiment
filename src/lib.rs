pub mod node;
pub mod style_structs;
pub mod ui_bundle;

use bevy::prelude::*;

pub use node::*;
pub use style_structs::*;
pub use ui_bundle::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {}
}
