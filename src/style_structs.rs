use std::hash::Hash;
use std::marker::PhantomData;

use bevy::{
    prelude::{info, Color, Component, Handle, Visibility},
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

    fn align_items(mut self, val: AlignItems) -> Self {
        self.get_component(move |v| v.align_items = val)
    }

    fn align_self(mut self, val: AlignSelf) -> Self {
        self.get_component(move |v| v.align_self = val)
    }

    fn align_content(mut self, val: AlignContent) -> Self {
        self.get_component(move |v| v.align_content = val)
    }

    fn justify_content(mut self, val: JustifyContent) -> Self {
        self.get_component(move |v| v.justify_content = val)
    }

    fn position(mut self, val: UiRect) -> Self {
        self.get_component(move |v| v.position = val)
    }

    fn margin(mut self, val: UiRect) -> Self {
        self.get_component(move |v| v.margin = val)
    }

    fn padding(mut self, val: UiRect) -> Self {
        self.get_component(move |v| v.padding = val)
    }

    fn border(mut self, val: UiRect) -> Self {
        self.get_component(move |v| v.border = val)
    }

    fn flex_grow(mut self, val: f32) -> Self {
        self.get_component(move |v| v.flex_grow = val)
    }

    fn flex_shrink(mut self, val: f32) -> Self {
        self.get_component(move |v| v.flex_shrink = val)
    }

    fn flex_basis(mut self, val: Val) -> Self {
        self.get_component(move |v| v.flex_basis = val)
    }

    fn size(mut self, val: Size) -> Self {
        self.get_component(move |v| v.size = val)
    }

    fn min_size(mut self, val: Size) -> Self {
        self.get_component(move |v| v.min_size = val)
    }

    fn max_size(mut self, val: Size) -> Self {
        self.get_component(move |v| v.max_size = val)
    }

    fn aspect_ratio(mut self, val: Option<f32>) -> Self {
        self.get_component(move |v| v.aspect_ratio = val)
    }

    fn overflow(mut self, val: Overflow) -> Self {
        self.get_component(move |v| v.overflow = val)
    }
    fn gap(mut self, val: Size) -> Self {
        self.get_component(move |v| v.gap = val)
    }
}

impl<T: StyleComponentApplier<Style> + Sized> Layout for T {}

pub trait FocusPolicyApplier: StyleComponentApplier<FocusPolicy> + Sized {
    fn focus_policy(mut self, val: FocusPolicy) -> Self {
        self.get_component(move |v| v.clone_from(&val))
    }
}

impl<T: StyleComponentApplier<FocusPolicy> + Sized> FocusPolicyApplier for T {}

pub trait ZIndexApplier: StyleComponentApplier<ZIndex> + Sized {
    fn z_index(mut self, val: ZIndex) -> Self {
        self.get_component(move |v| {
            v.clone_from(&val);
        })
    }
}

impl<T: StyleComponentApplier<ZIndex> + Sized> ZIndexApplier for T {}
pub trait VisibilityApplier: StyleComponentApplier<Visibility> + Sized {
    fn z_index(mut self, val: Visibility) -> Self {
        self.get_component(move |v| {
            v.clone_from(&val);
        })
    }
}

impl<T: StyleComponentApplier<Visibility> + Sized> VisibilityApplier for T {}
