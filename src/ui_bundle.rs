use bevy::{ecs::system::EntityCommands, prelude::*};

use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use crate::{style_structs::StyleComponentApplier, NullStyler, Styler};

use crate::{ui_id::*, UiImageBundle, UiNodeBundle, UiTextBundle};

// #[derive(Bundle)]
// pub struct UiBundle<Element: Component + Clone, StyleBundle: Bundle> {
//     value: Element,
//     style: StyleBundle
// }

// pub trait UiBundleGenerator<Element: Component + Clone> {
//     fn ui(self) -> UiBundle<Element, ()>;
// }

// impl<Element: Component + Clone> UiBundleGenerator<Element> for Element {
//     fn ui(self) -> UiBundle<Element, ()> {
//         UiBundle { value: self, style: () }
//     }
// }

// impl<Element: Component + Clone, StyleBundle: Bundle> UiBundle<Element, StyleBundle> {
//     pub fn background(self, c: Color) -> UiBundle<Element, (Styling<BackgroundColor, BgColor>, StyleBundle)> {
//         let t = BgColor(c).wrap();
//         UiBundle { value: self.value, style: (t, self.style) }
//     }
// }

pub trait UiBundleGenerator: Clone {
    fn spawn<'l, 'w, 's, 'a>(
        &self,
        commands: &'l mut EntityCommands<'w, 's, 'a>,
    ) -> &'l mut EntityCommands<'w, 's, 'a>;
}

pub trait UiBundleGeneratorStyler {
    fn apply_styler<S: Styler>(self, styler: &S) -> Self;
}

impl<T: Bundle + Clone + UiBundleGeneratorStyler> UiBundleGenerator for T {
    fn spawn<'l, 'w, 's, 'a>(
        &self,
        commands: &'l mut EntityCommands<'w, 's, 'a>,
    ) -> &'l mut EntityCommands<'w, 's, 'a> {
        commands.insert(self.clone())
    }
}

impl<
        'w,
        's,
        'a,
        Inner: Default,
        Bg: UiBundleGenerator + UiBundleGeneratorStyler + StyleComponentApplier<Inner>,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy,
    > StyleComponentApplier<Inner> for UiComponent<'w, 's, 'a, Bg, S, St, Id>
{
    fn get_component<T: FnMut(&mut Inner)>(mut self, apply: T) -> Self {
        let value = self.value.clone();
        self.value = value.get_component(apply);
        self
    }
}

pub struct UiComponent<
    'w,
    's,
    'a,
    T: UiBundleGenerator + UiBundleGeneratorStyler,
    S: InternalUiSpawner<'w, 's>,
    St: Styler,
    Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
> {
    pub value: T,
    spawner: Option<&'a mut S>,
    phantom: PhantomData<&'w T>,
    phantom_2: PhantomData<&'s T>,
    styler: Arc<St>,
    id: Option<Id>,
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > UiComponent<'w, 's, 'a, T, S, St, Id>
{
    pub fn new(value: T, spawner: &'a mut S, styler: Arc<St>) -> Self {
        let result = Self {
            value,
            spawner: Some(spawner),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler,
            id: None,
        };
        result.style_with_styler()
    }

    pub fn style<StB: Styler + 'static>(
        mut self,
        styler: StB,
    ) -> UiComponent<'w, 's, 'a, T, S, StB, Id> {
        let styler = styler;
        let styler: Arc<StB> = Arc::new(styler);
        let result = UiComponent {
            value: self.value.clone(),
            spawner: self.spawner.take(),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler,
            id: self.id,
        };
        result.style_with_styler()
    }

    pub fn id<IdB: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static>(
        mut self,
        id: IdB,
    ) -> UiComponent<'w, 's, 'a, T, S, St, IdB> {
        let id = id;
        UiComponent {
            value: self.value.clone(),
            spawner: self.spawner.take(),
            phantom: PhantomData,
            phantom_2: PhantomData,
            styler: self.styler.clone(),
            id: Some(id),
        }
    }

    fn style_with_styler(mut self) -> Self {
        self.value = self.value.clone().apply_styler(self.styler.as_ref());
        self
    }
}

pub trait UiComponentSpawner<T: UiBundleGenerator> {
    fn update_value<F: FnMut(&mut T) -> &mut T>(self, updator: F) -> Self;
}

pub trait UiComponentSpawnerActivator<'w, 's, 'a, T, S, St: Styler> {
    fn apply_id(&self, commands: &mut EntityCommands);
    fn get_component_styler(&self) -> Arc<St>;
    fn spawn(self) -> Option<EntityCommands<'w, 's, 'a>>;
    fn with_children<F: FnOnce((&mut ChildBuilder<'_, '_, '_>, Arc<St>))>(
        self,
        f: F,
    ) -> Option<EntityCommands<'w, 's, 'a>>
    where
        Self: Sized,
    {
        let styler = self.get_component_styler();
        let mut commands = self.spawn();
        if let Some(commands) = &mut commands {
            commands.with_children(move |builder| f((builder, styler)));
        }
        commands
    }
}

pub trait InternalUiSpawner<'w, 's>: Sized {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a>;

    fn spawn_ui_component<'a, T: UiBundleGenerator>(
        &'a mut self,
        value: &T,
    ) -> EntityCommands<'w, 's, 'a> {
        info!("Spawning UI Component...");
        let mut commands = self.spawn_empty();
        value.spawn(&mut commands);
        commands
    }
}

pub trait ExternalUiSpawner<'w, 's, St: Styler> {
    type InternalSpawner: InternalUiSpawner<'w, 's>;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner;
    fn get_styler(&self) -> Arc<St>;

    fn node<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, UiNodeBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(UiNodeBundle::default(), self.get_spawner(), styler)
    }

    fn text<'a>(
        &'a mut self,
        text: impl Into<String>,
    ) -> UiComponent<'w, 's, 'a, UiTextBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(
            UiTextBundle {
                node_bundle: TextBundle {
                    text: Text::from_section(text, TextStyle::default()),
                    ..default()
                },
                ..Default::default()
            },
            self.get_spawner(),
            styler,
        )
    }

    fn raw_text<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, UiTextBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(UiTextBundle::default(), self.get_spawner(), styler)
    }

    fn image<'a>(
        &'a mut self,
        image: Handle<Image>,
    ) -> UiComponent<'w, 's, 'a, UiImageBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(
            UiImageBundle {
                node_bundle: ImageBundle {
                    image: UiImage {
                        texture: image,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            self.get_spawner(),
            styler,
        )
    }

    fn button<'a>(
        &'a mut self,
    ) -> UiComponent<'w, 's, 'a, ButtonBundle, Self::InternalSpawner, St, usize> {
        let styler = self.get_styler();
        UiComponent::new(ButtonBundle::default(), self.get_spawner(), styler)
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy,
    > UiComponentSpawner<T> for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn update_value<U: FnMut(&mut T) -> &mut T>(mut self, mut updator: U) -> Self {
        updator(&mut self.value);
        self
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > UiComponentSpawnerActivator<'w, 's, 'a, T, S, St> for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn spawn(mut self) -> Option<EntityCommands<'w, 's, 'a>> {
        let id = self.id.take();
        let spawner = self.spawner.take();
        spawner.map(|spawner| {
            let mut result = spawner.spawn_ui_component(&self.value);
            if let Some(id) = id {
                result.insert(UiId::new(id));
            }
            result
        })
    }

    fn get_component_styler(&self) -> Arc<St> {
        self.styler.clone()
    }

    fn apply_id(&self, commands: &mut EntityCommands) {
        if let Some(id) = self.id.as_ref() {
            commands.insert(UiId::new(*id));
        }
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.spawn_empty()
    }
}

impl<'w, 's> ExternalUiSpawner<'w, 's, NullStyler> for Commands<'w, 's> {
    type InternalSpawner = Self;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner {
        self
    }

    fn get_styler(&self) -> Arc<NullStyler> {
        Arc::new(NullStyler)
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for ChildBuilder<'w, 's, '_> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.spawn_empty()
    }
}

impl<'w, 's, 'l, St: Styler> ExternalUiSpawner<'w, 's, St>
    for (&mut ChildBuilder<'w, 's, 'l>, Arc<St>)
{
    type InternalSpawner = ChildBuilder<'w, 's, 'l>;

    fn get_spawner(&mut self) -> &mut Self::InternalSpawner {
        self.0
    }

    fn get_styler(&self) -> Arc<St> {
        self.1.clone()
    }
}

impl<
        'w,
        's,
        'a,
        T: UiBundleGenerator + UiBundleGeneratorStyler,
        S: InternalUiSpawner<'w, 's>,
        St: Styler,
        Id: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy + 'static,
    > Drop for UiComponent<'w, 's, 'a, T, S, St, Id>
{
    fn drop(&mut self) {
        let id = self.id.take();
        let spawner = self.spawner.take();
        spawner.map(|spawner| {
            let mut result = spawner.spawn_ui_component(&self.value);
            if let Some(id) = id {
                result.insert(UiId::new(id));
            }
        });
    }
}
