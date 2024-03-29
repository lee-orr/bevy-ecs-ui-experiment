use bevy::{
    prelude::{Bundle, Component, ImageBundle, Mut, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, UiImage, ZIndex},
};

use crate::{style::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler};

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

#[derive(Bundle, Default)]
pub struct UiImageBundle {
    pub node_bundle: ImageBundle,
    pub marker: ImageNode,
}

impl Clone for UiImageBundle {
    fn clone(&self) -> Self {
        Self {
            node_bundle: ImageBundle {
                node: self.node_bundle.node,
                style: self.node_bundle.style.clone(),
                calculated_size: Default::default(),
                background_color: self.node_bundle.background_color,
                image: self.node_bundle.image.clone(),
                image_size: self.node_bundle.image_size,
                focus_policy: self.node_bundle.focus_policy,
                transform: self.node_bundle.transform,
                global_transform: self.node_bundle.global_transform,
                visibility: self.node_bundle.visibility,
                computed_visibility: self.node_bundle.computed_visibility.clone(),
                z_index: self.node_bundle.z_index,
            },
            marker: self.marker.clone(),
        }
    }
}

impl StyleComponentApplier<BackgroundColor> for UiImageBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for UiImageBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for UiImageBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for UiImageBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for UiImageBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.visibility);
        self
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
