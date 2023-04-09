use bevy::text::BreakLineOn;
use bevy::text::Font;

use bevy::prelude::Handle;

use bevy::prelude::Color;

use bevy::text::Text;
use bevy::text::TextAlignment;
use bevy::text::TextSection;
use bevy::text::TextStyle;

use crate::Editable;
use crate::EditableOption;

use super::StyleComponentApplier;

#[derive(Clone, Default)]
pub struct TextStyleEditable {
    pub font: Option<Handle<Font>>,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
}

impl Editable<TextStyle> for TextStyleEditable {
    fn merge(&self, original: &TextStyle) -> TextStyle {
        TextStyle {
            font: self.font.unwrap_or(original.font),
            font_size: self.font_size.unwrap_or(original.font_size),
            color: self.color.unwrap_or(original.color),
        }
    }
}

#[derive(Clone, Default)]
pub struct TextSectionEditable {
    pub value: Option<String>,
    pub style: Option<TextStyleEditable>,
}

impl Editable<TextSection> for TextSectionEditable {
    fn merge(&self, TextSection { value, style }: &TextSection) -> TextSection {
        TextSection {
            value: self.value.unwrap_or(value.clone()),
            style: self.style.realize(style),
        }
    }
}

#[derive(Clone, Default)]
pub struct TextEditable {
    pub sections: Option<Vec<TextSectionEditable>>,
    pub alignment: Option<TextAlignment>,
    pub linebreak_behaviour: Option<BreakLineOn>,
}

impl Editable<Text> for TextEditable {
    fn merge(&self, Text { sections, alignment, linebreak_behaviour }: &Text) -> Text {
        Text {
            sections: sections.clone(),
            alignment: self.alignment.unwrap_or(alignment.clone()),
            linebreak_behaviour: self.linebreak_behaviour.unwrap_or(linebreak_behaviour.clone()),
        }
    }
}

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
pub trait TextApplier: StyleComponentApplier<Text> + Sized {
    fn set_text(self, value: impl Into<String>) -> Self {
        let value = value.into();
        self.get_component(move |v| {
            let style = v
                .sections
                .first()
                .map(|a| a.style.clone())
                .unwrap_or_default();
            let text = Text::from_section(value.clone(), style);
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
