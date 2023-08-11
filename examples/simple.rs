use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(EcsUiPlugin)
        .add_plugin(UiPlugin::<MyUi>::new().initialize("simple.buml", "simple.css"))
        .add_startup_system(setup)
        .add_system(adjust_style)
        .run();
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct MyUi;

fn setup(mut commands: Commands, _assets: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MyUi);
}

fn adjust_style(_input: Res<Input<KeyCode>>) {}
