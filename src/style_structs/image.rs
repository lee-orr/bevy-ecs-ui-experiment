use bevy::prelude::Image;

use bevy::prelude::Handle;
use bevy::ui::UiImage;

use super::StyleComponentApplier;

pub trait ImageApplier: StyleComponentApplier<UiImage> + Sized {
    fn texture(self, val: Handle<Image>) -> Self {
        self.get_component(move |v| {
            v.texture = val.clone();
        })
    }

    fn flip(self, x: bool, y: bool) -> Self {
        self.get_component(move |v| {
            v.flip_x = x;
            v.flip_y = y;
        })
    }
}

impl<T: StyleComponentApplier<UiImage> + Sized> ImageApplier for T {}
