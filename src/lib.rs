pub mod button;
pub mod image;
pub mod node;
pub mod style_structs;
pub mod text;
pub mod ui_bundle;
pub mod ui_id;

use bevy::prelude::*;

pub use button::*;
pub use image::*;
pub use node::*;
pub use style_structs::*;
pub use text::*;
pub use ui_bundle::*;
pub use ui_id::*;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, _app: &mut App) {}
}
