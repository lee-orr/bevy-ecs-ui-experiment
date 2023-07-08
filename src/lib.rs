pub mod expression;
pub mod logical_nodes;
pub mod reactive_expression_handlers;
pub mod string_expression;
pub mod ui_asset;
pub mod ui_plugin;

use bevy::prelude::*;

use bevy_common_assets::xml::XmlAssetPlugin;
use tomt_bevycss::prelude::BevyCssPlugin;

pub use expression::*;
pub use ui_asset::UiNode;
use ui_asset::UiNodeTree;
pub use ui_plugin::{UIState, UiPlugin};

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyCssPlugin::default())
            .add_plugin(XmlAssetPlugin::<UiNodeTree>::new(&["buml"]));
    }
}

pub type ExpressionValue = rhai::Dynamic;
