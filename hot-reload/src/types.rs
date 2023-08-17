use std::path::PathBuf;

use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::*,
    utils::{HashMap, HashSet, Instant},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::InternalHotReload;
use crate::{reload_systems::*, replacable_types::*, schedules::*};

#[derive(Resource, Default, Reflect)]
pub struct HotReload {
    pub last_updated_frame: usize,
    pub version: usize,
    pub updated_this_frame: bool,
}

#[derive(Debug, Event, Reflect)]
pub struct HotReloadEvent {
    pub last_update_time: Instant,
}

#[derive(Debug, Default, Reflect)]
pub struct HotReloadOptions {
    pub lib_name: Option<String>,
    pub watch_folder: Option<PathBuf>,
    pub target_folder: Option<PathBuf>,
}
