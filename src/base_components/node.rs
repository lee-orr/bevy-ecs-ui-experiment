use bevy::{
    prelude::{Bundle, Component, Mut, NodeBundle, Visibility},
    ui::{BackgroundColor, FocusPolicy, Style, ZIndex},
};

use crate::{style_structs::StyleComponentApplier, UIQuery, UiBundleGeneratorStyler, UiBundleGenerator};

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

#[derive(Clone, Default)]
pub struct UiNodeBundleGenerator {
    style: Option<Style>,
    background_color: Option<BackgroundColor>,
    focus_policy: Option<FocusPolicy>,
    z_index: Option<ZIndex>,
    visibility: Option<Visibility>,
}

pub trait BaseNodeGenerator<Inner: Default, Applier: StyleComponentApplier<Inner>> {
    type Inner: Default;
    fn get_base_generator(&self) -> &Applier;
    fn get_base_generator_component<T: FnMut(&mut Self::Inner)>(&mut self, apply: T);
}

impl<R, Inner: Default> StyleComponentApplier<Inner> for R where R: BaseNodeGenerator<Inner, UiNodeBundleGenerator>, UiNodeBundleGenerator: StyleComponentApplier<Inner> {
    fn get_component<T: FnMut(&mut Inner)>(mut self, apply: T) -> Self {
        todo!()
    }
}

impl UiBundleGenerator for UiNodeBundleGenerator {
    fn spawn<'l, 'w, 's, 'a>(
        &self,
        commands: &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a>,
    ) -> &'l mut bevy::ecs::system::EntityCommands<'w, 's, 'a> {
        let mut bundle = NodeBundle::default();

        if let Some(style) = &self.style {
            bundle.style = style.clone();
        }
        if let Some(background_color) = self.background_color {
            bundle.background_color = background_color;
        }
        if let Some(focus_policy) = self.focus_policy {
            bundle.focus_policy = focus_policy;
        }
        if let Some(z_index) = self.z_index {
            bundle.z_index = z_index;
        }
        if let Some(visibility) = self.visibility {
            bundle.visibility = visibility;
        }

        commands.insert((UiNode, bundle))
    }
}

impl StyleComponentApplier<BackgroundColor> for UiNodeBundleGenerator {
    fn get_component<T: FnMut(&mut BackgroundColor)>(mut self, mut apply: T) -> Self {
        apply(&mut self.background_color.get_or_insert(BackgroundColor::DEFAULT));
        self
    }
}

impl StyleComponentApplier<Style> for UiNodeBundleGenerator {
    fn get_component<T: FnMut(&mut Style)>(mut self, mut apply: T) -> Self {
        apply(&mut self.style.get_or_insert(Style::default()));
        self
    }
}

impl StyleComponentApplier<FocusPolicy> for UiNodeBundleGenerator {
    fn get_component<T: FnMut(&mut FocusPolicy)>(mut self, mut apply: T) -> Self {
        apply(&mut self.focus_policy.get_or_insert(FocusPolicy::default()));
        self
    }
}

impl StyleComponentApplier<ZIndex> for UiNodeBundleGenerator {
    fn get_component<T: FnMut(&mut ZIndex)>(mut self, mut apply: T) -> Self {
        apply(&mut self.z_index.get_or_insert(ZIndex::default()));
        self
    }
}

impl StyleComponentApplier<Visibility> for UiNodeBundleGenerator {
    fn get_component<T: FnMut(&mut Visibility)>(mut self, mut apply: T) -> Self {
        apply(&mut self.visibility.get_or_insert(Visibility::default()));
        self
    }
}

impl UiBundleGeneratorStyler for UiNodeBundleGenerator {
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
