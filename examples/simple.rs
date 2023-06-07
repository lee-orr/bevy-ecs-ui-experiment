use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EcsUiPlugin)
        .add_startup_system(setup)
        .add_system(adjust_style)
        .run();
}

fn setup(mut commands: Commands, assets: ResMut<AssetServer>) {
    let _font: Handle<Font> = assets.load("libre-baskerville/LibreBaskerville-Regular.ttf");
    let _image: Handle<Image> = assets.load("test-image.png");
    commands.spawn(Camera2dBundle::default());
}

fn adjust_style(_input: Res<Input<KeyCode>>) {}
