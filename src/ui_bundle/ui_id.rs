use std::fmt::Debug;
use std::hash::Hash;

use bevy::prelude::Component;
use bevy::reflect::{FromReflect, Reflect};

#[derive(Component, Debug, Clone, Copy, Reflect, FromReflect)]
pub struct UiId<T: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy>(T);

impl<T: Debug + PartialEq + Eq + Hash + Sync + Send + Clone + Copy> UiId<T> {
    pub fn val(&self) -> &T {
        &self.0
    }

    pub fn new(val: T) -> Self {
        Self(val)
    }
}
