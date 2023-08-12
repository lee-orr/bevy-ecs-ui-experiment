use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::{IntoSystemConfigs, Schedule},
};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReloadUi;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiEvent;
