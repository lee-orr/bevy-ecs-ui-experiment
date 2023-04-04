use bevy::{
    prelude::{info, NodeBundle, Visibility, TextBundle},
    text::{Text, TextSection, TextStyle},
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UiBundleGenerator};

#[derive(Clone, Default)]
pub struct TextBuilder {
    pub styles: Vec<TextStyle>,
    pub text_sections: Vec<(String, usize)>,
}

impl From<TextBuilder> for Text {
    fn from(value: TextBuilder) -> Self {
        let default_style = value.styles.first().cloned().unwrap_or_default();

        Text::from_sections(value.text_sections.iter().map(|(text, style_id)| {
            let style = value.styles.get(*style_id).unwrap_or(&default_style);
            TextSection::new(text, style.clone())
        }))
    }
}

impl From<String> for TextBuilder {
    fn from(value: String) -> Self {
        TextBuilder { styles: vec![TextStyle::default()], text_sections: vec![(value, 0)] }
    }
}

#[derive(Default, Clone)]
pub struct TextBundleBuilder {
    pub node: NodeBundle,
    pub text: TextBuilder,
}

impl TextBundleBuilder {
    pub fn new(text: impl Into<String>) -> Self{
        let text = text.into();
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

// impl UiBundleGenerator for TextBundle {
//     fn spawn<'l, 'w, 's, 'a>(
//         &self,
//         commands: &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a>,
//     ) -> &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a> {
//         commands.insert(self.clone())
//     }
// }

impl StyleComponentApplier<BackgroundColor> for TextBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        info!("Dispatching background color...");
        apply(&mut self.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for TextBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for TextBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for TextBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for TextBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.visibility);
        self
    }
}

impl StyleComponentApplier<Text> for TextBundle {
    fn get_component<T: FnMut(&mut Text)>(mut self, mut apply: T) -> Self {
        apply(&mut self.text);
        self
    }
}
