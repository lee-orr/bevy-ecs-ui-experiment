use bevy::{prelude::Color, ui::BackgroundColor};

use super::StyleComponentApplier;

pub trait BgColor: StyleComponentApplier<BackgroundColor> + Sized {
    fn bg(self, color: Color) -> Self {
        self.get_component(move |v| {
            v.0 = color;
        })
    }
}

impl<T: StyleComponentApplier<BackgroundColor> + Sized> BgColor for T {}
