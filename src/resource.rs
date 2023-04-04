use bevy::prelude::*;

use super::level_loader::{LevelList, Level};

#[derive(Default, Resource)]
pub struct LoadingState {
    pub level_list: Handle<LevelList>,
}

#[derive(Default, Resource)]
pub struct LevelHandle {
    pub handle: Handle<Level>,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Screen {
    #[default]
    Loading,
    MainMenu,
    Level,
}

