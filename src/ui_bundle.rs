use bevy::{ecs::system::EntityCommands, prelude::*};

use std::marker::PhantomData;

use crate::{style_structs::StyleComponentApplier, TextBundleBuilder};

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

impl<T: Bundle + Clone> UiBundleGenerator for T {
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
        Bg: UiBundleGenerator + StyleComponentApplier<Inner>,
        S: InternalUiSpawner<'w, 's>,
    > StyleComponentApplier<Inner> for UiComponent<'w, 's, 'a, Bg, S>
{
    fn get_component<T: FnMut(&mut Inner)>(mut self, apply: T) -> Self {
        let value = self.value.clone();
        self.value = value.get_component(apply);
        self
    }
}

pub struct UiComponent<'w, 's, 'a, T: UiBundleGenerator, S: InternalUiSpawner<'w, 's>> {
    pub value: T,
    spawner: Option<&'a mut S>,
    phantom: PhantomData<&'w T>,
    phantom_2: PhantomData<&'s T>,
}

impl<'w, 's, 'a, T: UiBundleGenerator, S: InternalUiSpawner<'w, 's>> UiComponent<'w, 's, 'a, T, S> {
    pub fn new(value: T, spawner: &'a mut S) -> Self {
        Self {
            value,
            spawner: Some(spawner),
            phantom: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

pub trait UiComponentSpawner<T: UiBundleGenerator> {
    fn update_value<F: FnMut(&mut T) -> &mut T>(self, updator: F) -> Self;
}

pub trait UiComponentSpawnerActivator<'w, 's, 'a, T, S> {
    fn spawn(self) -> Option<EntityCommands<'w, 's, 'a>>;
    fn with_children<F: FnOnce(&mut bevy::prelude::ChildBuilder<'_, '_, '_>)>(
        self,
        f: F,
    ) -> Option<EntityCommands<'w, 's, 'a>>
    where
        Self: Sized,
    {
        let mut commands = self.spawn();
        if let Some(commands) = &mut commands {
            commands.with_children(move |builder| f(builder));
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

    fn node<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, NodeBundle, Self> {
        UiComponent::new(NodeBundle::default(), self)
    }

    fn text<'a>(&'a mut self, text: impl Into<String>) -> UiComponent<'w, 's, 'a, TextBundle, Self> {
        UiComponent::new(TextBundle { text: Text::from_section(text, TextStyle::default()), ..Default::default() }, self)
    }

    fn raw_text<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, TextBundle, Self> {
        UiComponent::new(TextBundle::default(), self)
    }

    // fn ui_root<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, UiRoot, Self>
    // where
    //     Self: Sized,
    // {
    //     UiComponent::new(UiRoot::new(), self)
    // }
    // fn div<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, Div, Self>
    // where
    //     Self: Sized,
    // {
    //     UiComponent::new(Div::new(), self)
    // }
    // fn text<'a, T: Into<String>>(&'a mut self, text: T) -> UiComponent<'w, 's, 'a, GameText, Self>
    // where
    //     Self: Sized,
    // {
    //     UiComponent::new(GameText::new(text), self)
    // }
    // fn button<'a, N: Into<String>, T: Into<String>>(
    //     &'a mut self,
    //     name: N,
    //     text: T,
    // ) -> UiComponent<'w, 's, 'a, GameButton, Self>
    // where
    //     Self: Sized,
    // {
    //     UiComponent::new(GameButton::new(name, text), self)
    // }

    // fn icon<'a>(&'a mut self, icon: Handle<Image>) -> UiComponent<'w, 's, 'a, GameIcon, Self>
    // where
    //     Self: Sized,
    // {
    //     UiComponent::new(GameIcon::new(icon), self)
    // }
}

impl<'w, 's, 'a, T: UiBundleGenerator, S: InternalUiSpawner<'w, 's>> UiComponentSpawner<T>
    for UiComponent<'w, 's, 'a, T, S>
{
    fn update_value<U: FnMut(&mut T) -> &mut T>(mut self, mut updator: U) -> Self {
        updator(&mut self.value);
        self
    }
}

impl<'w, 's, 'a, T: UiBundleGenerator, S: InternalUiSpawner<'w, 's>>
    UiComponentSpawnerActivator<'w, 's, 'a, T, S> for UiComponent<'w, 's, 'a, T, S>
{
    fn spawn(mut self) -> Option<EntityCommands<'w, 's, 'a>> {
        let spawner = self.spawner.take();
        spawner.map(|spawner| spawner.spawn_ui_component(&self.value))
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.spawn_empty()
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for ChildBuilder<'w, 's, '_> {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.spawn_empty()
    }
}

impl<'w, 's, 'a, T: UiBundleGenerator, S: InternalUiSpawner<'w, 's>> Drop
    for UiComponent<'w, 's, 'a, T, S>
{
    fn drop(&mut self) {
        if let Some(spawner) = self.spawner.take() {
            spawner.spawn_ui_component(&self.value);
        } else {
        }
    }
}

pub mod ui_id {
    use std::fmt::Debug;
    use std::hash::Hash;

    use bevy::prelude::Component;

    #[derive(Component, Debug)]
    pub struct UiId<T: Debug + PartialEq + Eq + Hash + Sync + Send>(T);

    impl<T: Debug + PartialEq + Eq + Hash + Sync + Send> UiId<T> {
        pub fn val(&self) -> &T {
            &self.0
        }

        pub fn new(val: T) -> Self {
            Self(val)
        }
    }
}
