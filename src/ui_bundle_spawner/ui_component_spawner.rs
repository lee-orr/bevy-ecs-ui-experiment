use std::sync::Arc;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{BuildChildren, ChildBuilder},
};

use crate::{Styler, UiBundleGenerator};

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
