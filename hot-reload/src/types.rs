use bevy::app::{App, AppLabel};
pub trait ReloadableComponent {}

pub trait ReloadableResource {}

#[derive(AppLabel)]
pub struct ReloadableApp;

pub trait ReloadablePlugin {
    fn construct(&self, app: &mut ReloadableApp) {}
}
