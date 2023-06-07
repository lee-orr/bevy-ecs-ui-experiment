pub mod ui_asset;

use bevy::prelude::*;

use bevy_common_assets::xml::XmlAssetPlugin;
use bevy_ecss::EcssPlugin;
use ui_asset::UiNode;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EcssPlugin::default())
            .add_plugin(XmlAssetPlugin::<UiNode>::new(&["buml"]));
    }
}
