use bevy::{
    prelude::{Bundle, Component, ImageBundle, Mut, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, UiImage, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler, UiNodeBundleGenerator, BaseNodeGenerator, UiBundleGenerator};

pub type ImageComponents<'a> = (
    &'a mut Style,
    &'a mut BackgroundColor,
    &'a mut FocusPolicy,
    &'a mut ZIndex,
    &'a mut Visibility,
    &'a mut UiImage,
);

pub type ImageQuery<'w, 's, 'a, T> = UIQuery<'w, 's, 'a, T, ImageComponents<'a>, ImageNode>;

#[derive(Component, Clone, Default)]
pub struct ImageNode;

#[derive(Clone, Default)]
pub struct UiImageBundle {
    pub base: UiNodeBundleGenerator,
    pub image: UiImage
}

impl<Inner: Default> BaseNodeGenerator<Inner,UiNodeBundleGenerator> for UiImageBundle where UiNodeBundleGenerator: StyleComponentApplier<Inner> {
    type Inner = Inner;
    fn get_base_generator(&self) -> &UiNodeBundleGenerator {
        &self.base
    }

    fn get_base_generator_component<T: FnMut(&mut Self::Inner)>(&mut self, apply: T) {
        todo!()
    }
}


impl UiBundleGenerator for UiImageBundle {
    fn spawn<'l, 'w, 's, 'a>(
        &self,
        commands: &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a>,
    ) -> &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a> {
        self.base.spawn(commands).insert(self.image.clone())
    }
}

impl StyleComponentApplier<UiImage> for UiImageBundle {
    fn get_component<T: FnMut(&mut UiImage)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.image);
        self
    }
}

impl UiBundleGeneratorStyler for UiImageBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}

impl<'a> StyleComponentApplier<BackgroundColor>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.1);
        self
    }
}

impl<'a> StyleComponentApplier<Style>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.0);
        self
    }
}

impl<'a> StyleComponentApplier<FocusPolicy>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.2);
        self
    }
}

impl<'a> StyleComponentApplier<ZIndex>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.3);
        self
    }
}

impl<'a> StyleComponentApplier<Visibility>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.4);
        self
    }
}

impl<'a> StyleComponentApplier<UiImage>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn get_component<T: FnMut(&mut UiImage)>(mut self, mut apply: T) -> Self {
        apply(&mut self.5);
        self
    }
}

impl<'a> UiBundleGeneratorStyler
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, UiImage>,
    )
{
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}
