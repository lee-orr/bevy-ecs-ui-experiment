use bevy::{
    prelude::{Bundle, ButtonBundle, Component, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, UiImage, ZIndex},
};

use crate::{style::StyleComponentApplier, UiBundleGeneratorStyler};

#[derive(Component)]
pub struct ButtonNode;

#[derive(Component)]
pub struct ClickedEventEmitter<T: Send + Sync>(T);

#[derive(Bundle)]
pub struct UiButtonBundle<T: Send + Sync + 'static> {
    node_bundle: ButtonBundle,
    marker: ButtonNode,
    clicked: ClickedEventEmitter<T>,
}

impl StyleComponentApplier<BackgroundColor> for ButtonBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for ButtonBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for ButtonBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for ButtonBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for ButtonBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.visibility);
        self
    }
}

impl StyleComponentApplier<UiImage> for ButtonBundle {
    fn get_component<T: FnMut(&mut UiImage)>(mut self, mut apply: T) -> Self {
        apply(&mut self.image);
        self
    }
}

impl UiBundleGeneratorStyler for ButtonBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}
