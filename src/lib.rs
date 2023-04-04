pub mod node;
pub mod style_structs;
pub mod text;
pub mod ui_bundle;

use bevy::prelude::*;

pub use node::*;
pub use style_structs::*;
pub use text::*;
pub use ui_bundle::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, _app: &mut App) {}
}
