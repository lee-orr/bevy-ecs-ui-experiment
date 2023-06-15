use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugin(WorldInspectorPlugin::default())
        .run();
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct MyUi;

fn setup(mut commands: Commands, _assets: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MyUi);
}

fn adjust_style(_input: Res<Input<KeyCode>>) {}
