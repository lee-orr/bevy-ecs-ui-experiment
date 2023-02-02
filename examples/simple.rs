use bevy::prelude::*;
use bevy_ecs_ui_experiment::ui_bundle::UiBundleGenerator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Text::from_section("test", TextStyle::default()).ui().background(Color::RED));
}