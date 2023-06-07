pub mod ui_asset;

use bevy::prelude::*;

use bevy_ecss::EcssPlugin;

pub struct EcsUiPlugin;

impl Plugin for EcsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EcssPlugin::default());
    }
}
