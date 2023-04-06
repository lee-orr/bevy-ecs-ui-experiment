use bevy::{
    prelude::{info, Color, Handle, Visibility, Image},
    text::{Font, Text, TextAlignment, TextSection, TextStyle},
    ui::*,
};

pub trait StyleComponentApplier<Inner: Default> {
    fn get_component<T: FnMut(&mut Inner)>(self, apply: T) -> Self;
}

pub trait BgColor: StyleComponentApplier<BackgroundColor> + Sized {
    fn bg(self, color: Color) -> Self {
        self.get_component(move |v| {
            info!("Setting background color {v:?} to {color:?}");
            v.0 = color;
            info!("set to {v:?}")
        })
    }
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

impl<T: StyleComponentApplier<BackgroundColor> + Sized> BgColor for T {}

pub trait TextStyling: StyleComponentApplier<TextStyle> + Sized {
    fn text_color(self, color: Color) -> Self {
        self.get_component(move |v| v.color = color)
    }
    fn font(self, font: Handle<Font>) -> Self {
        self.get_component(move |v| v.font = font.clone())
    }
    fn font_size(self, size: f32) -> Self {
        self.get_component(move |v| v.font_size = size)
    }
}
impl<T: StyleComponentApplier<TextStyle> + Sized> TextStyling for T {}

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
pub trait TextApplier: StyleComponentApplier<Text> + Sized {
    fn set_text(self, value: impl Into<String>) -> Self {
        let value = value.into();
        self.get_component(move |v| {
            let text = Text::from_section(value.clone(), TextStyle::default());
            v.clone_from(&text);
        })
    }

    fn set_style(self, value: TextStyle) -> Self {
        self.get_component(move |v| {
            if let Some(mut last) = v.sections.last_mut() {
                last.style = value.clone();
            }
        })
    }

    fn append_text(self, value: impl Into<String>) -> Self {
        let value: String = value.into();
        self.get_component(move |v| {
            let default_style = v.sections.first().cloned().unwrap_or_default();
            let text = TextSection::new(value.clone(), default_style.style);
            v.sections.push(text);
        })
    }

    fn text_color(self, value: Color) -> Self {
        self.get_component(move |v| {
            if let Some(mut last) = v.sections.last_mut() {
                last.style.color = value;
            }
        })
    }

    fn font_size(self, size: f32) -> Self {
        self.get_component(move |v| {
            if let Some(mut last) = v.sections.last_mut() {
                last.style.font_size = size;
            }
        })
    }

    fn font(self, font: Handle<Font>) -> Self {
        self.get_component(move |v| {
            if let Some(mut last) = v.sections.last_mut() {
                last.style.font = font.clone();
            }
        })
    }

    fn text_alignment(self, alignment: TextAlignment) -> Self {
        self.get_component(move |v| {
            v.alignment = alignment;
        })
    }
}

impl<T: StyleComponentApplier<Text> + Sized> TextApplier for T {}

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