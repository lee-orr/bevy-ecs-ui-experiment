use bevy::prelude::*;
use bevy_ecs_ui_experiment::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EcsUiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone)]
pub enum UiStyler {
    Base(Handle<Font>),
    Heading(Handle<Font>),
}

impl Styler for UiStyler {
    fn text_section_style<T: TextStyling>(&self, styled: T) -> T {
        match self {
            UiStyler::Base(font) => styled.font(font.clone()).font_size(25.),
            UiStyler::Heading(font) => styled
                .font(font.clone())
                .font_size(40.)
                .text_color(Color::BLUE),
        }
    }

    fn text_style<T: TextApplier>(&self, styled: T) -> T {
        match self {
            UiStyler::Base(font) => styled.font(font.clone()).font_size(25.),
            UiStyler::Heading(font) => styled
                .font(font.clone())
                .font_size(40.)
                .text_color(Color::BLUE),
        }
    }

    fn style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
    ) -> T {
        styled.padding(UiRect::all(Val::Px(10.)))
    }
}

fn setup(mut commands: Commands, assets: ResMut<AssetServer>) {
    let font: Handle<Font> = assets.load("libre-baskerville/LibreBaskerville-Regular.ttf");
    commands.spawn(Camera2dBundle::default());

    commands
        .node()
        .style(UiStyler::Base(font.clone()))
        .bg(Color::rgb(1., 0.5, 0.2))
        .size(Size::all(Val::Px(400.)))
        .with_children(|mut p| {
            p.text("test!").append_text("hello").text_color(Color::RED);
            p.node().bg(Color::GREEN).size(Size::all(Val::Px(200.)));
            p.node()
                .style(UiStyler::Heading(font.clone()))
                .with_children(|mut p| {
                    p.text("Header");
                });
        });
}
