use bevy::prelude::World;

use crate::{reload_systems::setup_reloadable_app, ReloadableSetup, SetupReload};

pub trait ReloadableAppSetup {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self;
}

impl ReloadableAppSetup for bevy::app::App {
    fn add_reloadables<T: ReloadableSetup>(&mut self) -> &mut Self {
        let name = T::setup_function_name();
        let system = move |world: &mut World| setup_reloadable_app::<T>(name, world);
        self.add_systems(SetupReload, system)
    }
}
