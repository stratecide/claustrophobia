use bevy::prelude::*;

use crate::resource::*;
use crate::level_loader::LevelList;

use super::medicine::resource::SideEffects;

pub fn next_level(
    mut next_level: ResMut<NextState<Screen>>,
    mut level_handle: ResMut<LevelHandle>,
    level_list: Res<Assets<LevelList>>,
    loading_state: Res<LoadingState>,
    mut side_effects: ResMut<SideEffects>,
) {
    if !level_handle.waited {
        level_handle.waited = true;
        return;
    }
    *side_effects = SideEffects::new();
    level_handle.waited = false;
    let level_list = level_list.get(&loading_state.level_list).unwrap();
    level_handle.handle = level_list.levels[level_handle.level_id].1.clone();
    if level_handle.level_id == level_list.levels.len() - 1 {
        side_effects.force_unsquish();
    }
    next_level.set(Screen::Level);
}

