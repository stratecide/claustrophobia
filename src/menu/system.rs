use bevy::prelude::*;

use crate::resource::*;
use crate::level_loader::*;

pub fn transition_to_level(
    keyboard: Res<Input<KeyCode>>,
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
    level_list: Res<Assets<LevelList>>,
    loading_state: Res<LoadingState>,
    mut level_handle: ResMut<LevelHandle>,
) {
    if screen.0 == Screen::MainMenu {
        let level_list = match level_list.get(&loading_state.level_list) {
            Some(ll) => ll,
            None => return,
        };
        /*let mut load_level = false;
        if keyboard.just_pressed(KeyCode::Key1) {
            level_handle.level_id = 0;
            load_level = true;
        } else if keyboard.just_pressed(KeyCode::Key2) {
            level_handle.level_id = 1;
            load_level = true;
        } else if keyboard.just_pressed(KeyCode::Key3) {
            level_handle.level_id = 2;
            load_level = true;
        } else if keyboard.just_pressed(KeyCode::Key4) {
            level_handle.level_id = 3;
            load_level = true;
        } else if keyboard.just_pressed(KeyCode::Key5) {
            level_handle.level_id = 4;
            load_level = true;
        }
        if load_level {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[level_handle.level_id].1.clone();
            next_screen.set(Screen::Level);
        }*/
        level_handle.handle = level_list.levels[level_handle.level_id].1.clone();
        next_screen.set(Screen::NextLevel);
    }
}

