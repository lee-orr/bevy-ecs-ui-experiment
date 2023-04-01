use std::hash::Hash;
use std::marker::PhantomData;

use bevy::{
    prelude::{info, Color, Component, Handle},
    text::{Font, TextStyle},
    ui::*,
};

pub trait StyleComponentApplier<Inner: Default> {
    fn get_component<T: FnMut(&mut Inner) -> ()>(self, apply: T) -> Self;
}

pub trait BgColor: StyleComponentApplier<BackgroundColor> + Sized {
    fn bg(mut self, color: Color) -> Self {
        self.get_component(move |v| {
            info!("Setting background color {v:?} to {color:?}");
            v.0 = color;
            info!("set to {v:?}")
        })
    }
}

impl<T: StyleComponentApplier<BackgroundColor> + Sized> BgColor for T {}

pub trait TextStyling: StyleComponentApplier<TextStyle> + Sized {
    fn text_color(mut self, color: Color) -> Self {
        self.get_component(move |v| v.color = color)
    }
    fn font(mut self, font: Handle<Font>) -> Self {
        self.get_component(move |v| v.font = font.clone())
    }
    fn font_size(mut self, size: f32) -> Self {
        self.get_component(move |v| v.font_size = size)
    }
}

pub trait Layout: StyleComponentApplier<Style> + Sized {
    fn display(mut self, val: Display) -> Self {
        self.get_component(move |v| v.display = val)
    }
    fn position_type(mut self, val: PositionType) -> Self {
        self.get_component(move |v| v.position_type = val)
    }
    fn direction(mut self, val: Direction) -> Self {
        self.get_component(move |v| v.direction = val)
    }

    fn flex_direction(mut self, val: FlexDirection) -> Self {
        self.get_component(move |v| v.flex_direction = val)
    }

    fn flex_wrap(mut self, val: FlexWrap) -> Self {
        self.get_component(move |v| v.flex_wrap = val)
    }

    fn size(mut self, val: Size) -> Self {
        self.get_component(move |v| v.size = val)
    }
}

impl<T: StyleComponentApplier<Style> + Sized> Layout for T {}
