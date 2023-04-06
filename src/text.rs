use bevy::{
    prelude::{info, TextBundle, Visibility},
    text::Text,
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UiBundleGeneratorStyler};

impl StyleComponentApplier<BackgroundColor> for TextBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        info!("Dispatching background color...");
        apply(&mut self.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for TextBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for TextBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for TextBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for TextBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.visibility);
        self
    }
}

impl StyleComponentApplier<Text> for TextBundle {
    fn get_component<T: FnMut(&mut Text)>(mut self, mut apply: T) -> Self {
        apply(&mut self.text);
        self
    }
}

impl UiBundleGeneratorStyler for TextBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        let s = styler.text_style(self);
        styler.style(s)
    }
}
