pub mod expression;
pub mod reactive_expression_handlers;
pub mod string_expression;
pub mod ui_asset;
pub mod ui_plugin;

use bevy::prelude::*;

use bevy_common_assets::xml::XmlAssetPlugin;
use bevy_ecss::EcssPlugin;

pub use expression::*;
pub use ui_asset::UiNode;
pub use ui_plugin::{UIState, UiPlugin};

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EcssPlugin::default())
            .add_plugin(XmlAssetPlugin::<UiNode>::new(&["buml"]));
    }
}
