use bevy::{prelude::Visibility, ui::*};

use super::StyleComponentApplier;

pub trait Layout: StyleComponentApplier<Style> + Sized {
    fn display(self, val: Display) -> Self {
        self.get_component(move |v| v.display = val)
    }
    fn position_type(self, val: PositionType) -> Self {
        self.get_component(move |v| v.position_type = val)
    }
    fn direction(self, val: Direction) -> Self {
        self.get_component(move |v| v.direction = val)
    }

    fn flex_direction(self, val: FlexDirection) -> Self {
        self.get_component(move |v| v.flex_direction = val)
    }

    fn flex_wrap(self, val: FlexWrap) -> Self {
        self.get_component(move |v| v.flex_wrap = val)
    }

    fn align_items(self, val: AlignItems) -> Self {
        self.get_component(move |v| v.align_items = val)
    }

    fn align_self(self, val: AlignSelf) -> Self {
        self.get_component(move |v| v.align_self = val)
    }

    fn align_content(self, val: AlignContent) -> Self {
        self.get_component(move |v| v.align_content = val)
    }

    fn justify_content(self, val: JustifyContent) -> Self {
        self.get_component(move |v| v.justify_content = val)
    }

    fn position(self, val: UiRect) -> Self {
        self.get_component(move |v| v.position = val)
    }

    fn margin(self, val: UiRect) -> Self {
        self.get_component(move |v| v.margin = val)
    }

    fn padding(self, val: UiRect) -> Self {
        self.get_component(move |v| v.padding = val)
    }

    fn border(self, val: UiRect) -> Self {
        self.get_component(move |v| v.border = val)
    }

    fn flex_grow(self, val: f32) -> Self {
        self.get_component(move |v| v.flex_grow = val)
    }

    fn flex_shrink(self, val: f32) -> Self {
        self.get_component(move |v| v.flex_shrink = val)
    }

    fn flex_basis(self, val: Val) -> Self {
        self.get_component(move |v| v.flex_basis = val)
    }

    fn size(self, val: Size) -> Self {
        self.get_component(move |v| v.size = val)
    }

    fn min_size(self, val: Size) -> Self {
        self.get_component(move |v| v.min_size = val)
    }

    fn max_size(self, val: Size) -> Self {
        self.get_component(move |v| v.max_size = val)
    }

    fn aspect_ratio(self, val: Option<f32>) -> Self {
        self.get_component(move |v| v.aspect_ratio = val)
    }

    fn overflow(self, val: Overflow) -> Self {
        self.get_component(move |v| v.overflow = val)
    }
    fn gap(self, val: Size) -> Self {
        self.get_component(move |v| v.gap = val)
    }
}

impl<T: StyleComponentApplier<Style> + Sized> Layout for T {}

pub trait FocusPolicyApplier: StyleComponentApplier<FocusPolicy> + Sized {
    fn focus_policy(self, val: FocusPolicy) -> Self {
        self.get_component(move |v| v.clone_from(&val))
    }
}

impl<T: StyleComponentApplier<FocusPolicy> + Sized> FocusPolicyApplier for T {}

pub trait ZIndexApplier: StyleComponentApplier<ZIndex> + Sized {
    fn z_index(self, val: ZIndex) -> Self {
        self.get_component(move |v| {
            v.clone_from(&val);
        })
    }
}

impl<T: StyleComponentApplier<ZIndex> + Sized> ZIndexApplier for T {}

pub trait VisibilityApplier: StyleComponentApplier<Visibility> + Sized {
    fn z_index(self, val: Visibility) -> Self {
        self.get_component(move |v| {
            v.clone_from(&val);
        })
    }
}

impl<T: StyleComponentApplier<Visibility> + Sized> VisibilityApplier for T {}
