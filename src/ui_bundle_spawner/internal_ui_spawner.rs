use bevy::{
    ecs::system::EntityCommands,
    prelude::{ChildBuilder, Commands},
};

use crate::UiBundleGenerator;

pub trait InternalUiSpawner<'w, 's>: Sized {
    fn spawn_empty<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a>;

    fn spawn_ui_component<'a, T: UiBundleGenerator>(
        &'a mut self,
        value: &T,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut commands = self.spawn_empty();
        value.spawn(&mut commands);
        commands
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
