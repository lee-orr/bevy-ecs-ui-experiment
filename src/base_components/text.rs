use bevy::{
    prelude::{Bundle, Component, Mut, TextBundle, Visibility},
    text::{Text, TextStyle},
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler, UiNodeBundleGenerator, BaseNodeGenerator, UiBundleGenerator, TextEditable, TextSectionEditable, EditableOption};

pub type TextComponents<'a> = (
    &'a mut Style,
    &'a mut BackgroundColor,
    &'a mut FocusPolicy,
    &'a mut ZIndex,
    &'a mut Visibility,
    &'a mut Text,
);

pub type TextQuery<'w, 's, 'a, T> = UIQuery<'w, 's, 'a, T, TextComponents<'a>, TextNode>;

#[derive(Component, Clone, Default)]
pub struct TextNode;

#[derive(Clone, Default)]
pub struct UiTextBundle {
    pub base: UiNodeBundleGenerator,
    pub text: Option<TextEditable>
}

impl UiTextBundle {
    pub fn new(text: impl Into<String>) -> Self {
        let mut text_editable = TextEditable::default();
        text_editable.sections = Some(vec![TextSectionEditable { value: Some(text.into()), ..Default::default()}]);
        UiTextBundle { base:  UiNodeBundleGenerator::default(), text: Some(text_editable) }
    }
}

impl<Inner: Default> BaseNodeGenerator<Inner,UiNodeBundleGenerator> for UiTextBundle where UiNodeBundleGenerator: StyleComponentApplier<Inner> {
    type Inner = Inner;
    fn get_base_generator(&self) -> &UiNodeBundleGenerator {
        &self.base
    }

    fn get_base_generator_component<T: FnMut(&mut Self::Inner)>(&mut self, apply: T) {
        todo!()
    }
}


impl UiBundleGenerator for UiTextBundle {
    fn spawn<'l, 'w, 's, 'a>(
        &self,
        commands: &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a>,
    ) -> &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a> {
        self.base.spawn(commands).insert(self.text.realize_default())
    }
}

impl StyleComponentApplier<TextEditable> for UiTextBundle {
    fn get_component<T: FnMut(&mut TextEditable)>(mut self, mut apply: T) -> Self {
        apply(&mut self.text.get_or_insert(TextEditable::default()));
        self
    }
}

impl UiBundleGeneratorStyler for UiTextBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        let s = styler.text_style(self);
        styler.style(s)
    }
}

impl<'a> StyleComponentApplier<BackgroundColor>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.1);
        self
    }
}

impl<'a> StyleComponentApplier<Style>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.0);
        self
    }
}

impl<'a> StyleComponentApplier<FocusPolicy>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.2);
        self
    }
}

impl<'a> StyleComponentApplier<ZIndex>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.3);
        self
    }
}

impl<'a> StyleComponentApplier<Visibility>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.4);
        self
    }
}

impl<'a> StyleComponentApplier<Text>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn get_component<T: FnMut(&mut Text)>(mut self, mut apply: T) -> Self {
        apply(&mut self.5);
        self
    }
}

impl<'a> UiBundleGeneratorStyler
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
        Mut<'a, Text>,
    )
{
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        let s = styler.text_style(self);
        styler.style(s)
    }
}
