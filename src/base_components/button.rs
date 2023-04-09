use bevy::{
    prelude::{Bundle, ButtonBundle, Component, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, UiImage, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UiBundleGeneratorStyler};

#[derive(Component)]
pub struct ButtonNode;

#[derive(Bundle)]
pub struct UiButtonBundle {
    node_bundle: ButtonBundle,
    marker: ButtonNode,
}

impl StyleComponentApplier<BackgroundColor> for UiButtonBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for UiButtonBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for UiButtonBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for UiButtonBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for UiButtonBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.visibility);
        self
    }
}

impl StyleComponentApplier<UiImage> for UiButtonBundle {
    fn get_component<T: FnMut(&mut UiImage)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.image);
        self
    }
}

impl UiBundleGeneratorStyler for UiButtonBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}
