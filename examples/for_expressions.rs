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
        .add_plugin(UiPlugin::<MyUi>::new().initialize("for_expressions.buml", "simple.css"))
        .add_startup_system(setup)
        .add_system(adjust_style)
        .add_plugin(WorldInspectorPlugin::default())
        .run();
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct MyUi {
    content: Vec<String>,
}

fn setup(mut commands: Commands, _assets: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MyUi {
        content: vec!["0".to_string(), "1".to_string()],
    });
}

fn adjust_style(input: Res<Input<KeyCode>>, mut ui: Query<&mut MyUi>) {
    if input.just_pressed(KeyCode::Return) {
        for mut ui in ui.iter_mut() {
            info!("Changin UI - {ui:?}");
            let next = ui.content.len().to_string();
            ui.content.push(next);
        }
    }
}
