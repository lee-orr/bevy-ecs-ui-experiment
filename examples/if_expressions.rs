use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(EcsUiPlugin)
        .add_plugin(UiPlugin::<MyUi>::new().initialize("if_expressions.buml", "simple.css"))
        .add_startup_system(setup)
        .add_system(adjust_style)
        .run();
}

#[derive(Component, Reflect, Clone, Debug)]
pub struct MyUi {
    index: usize,
}

fn setup(mut commands: Commands, _assets: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MyUi { index: 0 });
}

fn adjust_style(input: Res<Input<KeyCode>>, mut ui: Query<&mut MyUi>) {
    if input.just_pressed(KeyCode::Return) {
        for mut ui in ui.iter_mut() {
            info!("Changin UI - {ui:?}");
            ui.index += 1;
        }
    }
}
