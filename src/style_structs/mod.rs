pub mod background_color;
pub mod node;
pub mod text;
pub mod image;

pub use background_color::*;
pub use node::*;
pub use text::*;
pub use image::*;

use bevy::{
    prelude::{Component, },
    ui::*,
};

pub trait StyleComponentApplier<Inner: Default> {
    fn get_component<T: FnMut(&mut Inner)>(self, apply: T) -> Self;
}

pub trait TypedStyler<Input = ()> {
    fn typed_text_section_style<T: TextStyling>(&self, styled: T, input: Input) -> T;
    fn typed_text_style<T: TextApplier>(&self, styled: T, input: Input) -> T;
    fn typed_style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
        input: Input,
    ) -> T;
}

pub trait Styler {
    fn text_section_style<T: TextStyling>(&self, styled: T) -> T;
    fn text_style<T: TextApplier>(&self, styled: T) -> T;
    fn style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
    ) -> T;
}

impl<S: Styler> TypedStyler<()> for S {
    fn typed_text_section_style<T: TextStyling>(&self, styled: T, _: ()) -> T {
        self.text_section_style(styled)
    }

    fn typed_text_style<T: TextApplier>(&self, styled: T, _: ()) -> T {
        self.text_style(styled)
    }

    fn typed_style<T: Layout + VisibilityApplier + BgColor + FocusPolicyApplier + ZIndexApplier>(
        &self,
        styled: T,
        _: (),
    ) -> T {
        self.style(styled)
    }
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


pub trait RuntimeStyler<T>: Component + TypedStyler<T> {}

pub trait InteractionStyler: Component + TypedStyler<Interaction> {}

pub trait Editable<T: Default> : Clone + Default {
    fn merge(&self, original: &T) -> T;

    fn build(&self) -> T {
        let default = T::default();
        self.merge(&default)
    }
}

pub trait EditableOption<T: Default, R: Editable<T>> {
    fn realize(&self, input: &T) -> T;

    fn realize_default(&self) -> T {
        self.realize(&T::default())
    }
}

impl<T: Default + Clone, R: Editable<T>> EditableOption<T, R> for Option<R> {
    fn realize(&self, input: &T) -> T {
        if let Some(v) = self {
            v.merge(&input)
        } else {
            input.clone()
        }
    }
}