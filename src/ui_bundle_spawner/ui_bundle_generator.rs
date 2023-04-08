use crate::Styler;

use bevy::{ecs::system::EntityCommands, prelude::Bundle};

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
