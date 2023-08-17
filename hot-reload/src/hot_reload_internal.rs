use bevy::{prelude::Resource, utils::Instant};

use crate::{lib_set::LibPathSet, library_holder::LibraryHolder};

#[derive(Resource)]
pub struct InternalHotReload {
    pub library: Option<LibraryHolder>,
    pub last_lib: Option<LibraryHolder>,
    pub updated_this_frame: bool,
    pub last_update_time: Instant,
    pub libs: LibPathSet,
}
