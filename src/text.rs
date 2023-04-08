use bevy::{
    prelude::{info, Bundle, Component, Mut, TextBundle, Visibility},
    text::Text,
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler};

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

#[derive(Bundle, Clone, Default)]
pub struct UiTextBundle {
    pub node_bundle: TextBundle,
    pub marker: TextNode,
}

impl StyleComponentApplier<BackgroundColor> for UiTextBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        info!("Dispatching background color...");
        apply(&mut self.node_bundle.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for UiTextBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for UiTextBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for UiTextBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for UiTextBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.visibility);
        self
    }
}

impl StyleComponentApplier<Text> for UiTextBundle {
    fn get_component<T: FnMut(&mut Text)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.text);
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
        info!("Dispatching background color...");
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
