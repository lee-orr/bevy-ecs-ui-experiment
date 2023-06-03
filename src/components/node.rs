use bevy::{
    prelude::{Bundle, Component, Mut, NodeBundle, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler};

pub type NodeComponents<'a> = (
    &'a mut Style,
    &'a mut BackgroundColor,
    &'a mut FocusPolicy,
    &'a mut ZIndex,
    &'a mut Visibility,
);

pub type NodeQuery<'w, 's, 'a, T> = UIQuery<'w, 's, 'a, T, NodeComponents<'a>, UiNode>;

#[derive(Component, Clone, Default)]
pub struct UiNode;

#[derive(Bundle, Clone, Default)]
pub struct UiNodeBundle {
    node_bundle: NodeBundle,
    marker: UiNode,
}

impl StyleComponentApplier<BackgroundColor> for UiNodeBundle {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.background_color);
        self
    }
}

impl StyleComponentApplier<Style> for UiNodeBundle {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.style);
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for UiNodeBundle {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.focus_policy);
        self
    }
}

impl StyleComponentApplier<ZIndex> for UiNodeBundle {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.z_index);
        self
    }
}

impl StyleComponentApplier<Visibility> for UiNodeBundle {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.node_bundle.visibility);
        self
    }
}

impl UiBundleGeneratorStyler for UiNodeBundle {
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}

impl<'a> StyleComponentApplier<BackgroundColor>
    for (
        Mut<'a, Style>,
        Mut<'a, BackgroundColor>,
        Mut<'a, FocusPolicy>,
        Mut<'a, ZIndex>,
        Mut<'a, Visibility>,
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
    )
{
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.4);
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
    )
{
    fn apply_styler<S: crate::Styler>(self, styler: &S) -> Self {
        styler.style(self)
    }
}
