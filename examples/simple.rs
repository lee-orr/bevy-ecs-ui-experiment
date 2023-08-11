use bevy::prelude::*;
use bevy_ecs_ui_experiment::{ui_id::UiId, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EcsUiPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .register_type::<UiId<i32>>()
        .add_startup_system(setup)
        .add_system(adjust_style)
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
    let image: Handle<Image> = assets.load("test-image.png");
    commands.spawn(Camera2dBundle::default());

    commands
        .node()
        .style(UiStyler::Base(font.clone()))
        .bg(Color::rgb(1., 0.5, 0.2))
        .width(Val::Px(400.))
        .height(Val::Px(400.))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::SpaceBetween)
        .with_children(|mut p| {
            p.node()
                .style(UiStyler::Heading(font.clone()))
                .with_children(|mut p| {
                    p.text("HEADING!");
                });
            p.text("A IS NOT PRESSED").id(15);
            p.text("I don't change...").id(16);
            p.image(image)
                .width(Val::Px(150.))
                .height(Val::Px(150.))
                .id(16);
        });
}

fn adjust_style(
    mut query: ParamSet<(TextQuery<i32>, ImageQuery<i32>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::A) {
        for (id, node) in query.p0().iter_mut() {
            if *id.val() == 15 {
                node.bg(Color::GREEN)
                    .set_text("A Is Pressed")
                    .text_color(Color::BLACK);
            }
        }

        for (_, node) in query.p1().iter_mut() {
            node.flip(true, true);
        }
    } else {
        for (id, node) in query.p0().iter_mut() {
            if *id.val() == 15 {
                node.bg(Color::RED)
                    .set_text("A Is Not Pressed")
                    .text_color(Color::WHITE);
            }
        }

        for (_, node) in query.p1().iter_mut() {
            node.flip(false, false);
        }
    }
}
