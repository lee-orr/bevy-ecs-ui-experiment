use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EcsUiPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, assets: ResMut<AssetServer>) {
    let font: Handle<Font> = assets.load("libre-baskerville/LibreBaskerville-Regular.ttf");
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle::default().bg(Color::BLUE))
        .with_children(|p| {
            p.node()
                .bg(Color::rgb(1., 0.5, 0.2))
                .size(Size::all(Val::Px(400.)))
                .with_children(|p| {
                    p.text("test!").text_color(Color::GREEN).font_size(40.).font(font.clone());
                    p.spawn(TextBundle {
                        text: Text::from_section(
                            "test",
                            TextStyle {
                                font_size: 30.,
                                font: font.clone(),
                                color: Color::PURPLE,
                            },
                        ),
                        ..default()
                    });
                    p.spawn(NodeBundle {
                        style: Style {
                            size: Size::all(Val::Px(50.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::RED),
                        ..default()
                    });
                });
        });
}
