use bevy::{
    prelude::{info, NodeBundle},
    ui::{BackgroundColor, Style},
};

use crate::{style_structs::StyleComponentApplier, InternalUiSpawner};

impl StyleComponentApplier<BackgroundColor> for NodeBundle {
    fn get_component<T: FnMut(&mut BackgroundColor) -> ()>(mut self, mut apply: T) -> Self {
        info!("Dispatching background color...");
        apply(&mut self.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for NodeBundle {
    fn get_component<T: FnMut(&mut Style) -> ()>(mut self, mut apply: T) -> Self {
        apply(&mut self.style);
        self
    }
}
