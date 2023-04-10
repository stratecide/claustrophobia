use bevy::prelude::*;

use super::level_loader::{LevelList, Level};

#[derive(Default, Resource)]
pub struct LoadingState {
    pub level_list: Handle<LevelList>,
}

#[derive(Default, Resource)]
pub struct LevelHandle {
    pub level_id: usize,
    pub handle: Handle<Level>,
    pub waited: bool,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Screen {
    #[default]
    Loading,
    MainMenu,
    Level,
    NextLevel,
}

