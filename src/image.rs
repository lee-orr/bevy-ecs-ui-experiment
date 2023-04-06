use bevy::{
    prelude::{info, ImageBundle, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex, UiImage},
};

use crate::{style_structs::StyleComponentApplier, UiBundleGeneratorStyler};

impl StyleComponentApplier<BackgroundColor> for ImageBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        info!("Dispatching background color...");
        apply(&mut self.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for ImageBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for ImageBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for ImageBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for ImageBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.visibility);
        self
    }
}

impl StyleComponentApplier<UiImage> for ImageBundle {
    fn get_component<T: FnMut(&mut UiImage)>(mut self, mut apply: T) -> Self {
        apply(&mut self.image);
        self
    }
}

impl UiBundleGeneratorStyler for ImageBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}
