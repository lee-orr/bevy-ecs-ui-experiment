pub mod background_color;
pub mod node;
pub mod text;

pub use background_color::*;
pub use node::*;
pub use text::*;

use bevy::{
    prelude::{Handle, Image},
    ui::*,
};

pub trait StyleComponentApplier<Inner: Default> {
    fn get_component<T: FnMut(&mut Inner)>(self, apply: T) -> Self;
}

pub trait Styler {
    fn text_section_style<T: TextStyling>(&self, styled: T) -> T;
    fn text_style<T: TextApplier>(&self, styled: T) -> T;
    fn style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
    ) -> T;
}

pub struct NullStyler;
impl Styler for NullStyler {
    fn text_section_style<T: TextStyling>(&self, styled: T) -> T {
        styled
    }

    fn text_style<T: TextApplier>(&self, styled: T) -> T {
        styled
    }

    fn style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
    ) -> T {
        styled
    }
}

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
