use bevy::prelude::World;

use crate::{reload_systems::setup_reloadable_app, ReloadableAppContents, SetupReload};

pub trait ReloadableElementsSetup {
    fn setup_reloadable_elements<T: ReloadableSetup>(&mut self) -> &mut Self;
}

impl ReloadableElementsSetup for bevy::app::App {
    fn setup_reloadable_elements<T: ReloadableSetup>(&mut self) -> &mut Self {
        let name = T::setup_function_name();
        let system = move |world: &mut World| setup_reloadable_app::<T>(name, world);
        self.add_systems(SetupReload, system)
    }
}

pub trait ReloadableSetup {
    fn setup_function_name() -> &'static str;
    fn default_function(app: &mut ReloadableAppContents);
}
