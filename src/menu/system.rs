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
        if keyboard.just_pressed(KeyCode::Key1) {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[0].1.clone();
            next_screen.set(Screen::Level);
        } else if keyboard.just_pressed(KeyCode::Key2) {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[1].1.clone();
            next_screen.set(Screen::Level);
        } else if keyboard.just_pressed(KeyCode::Key3) {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[2].1.clone();
            next_screen.set(Screen::Level);
        } else if keyboard.just_pressed(KeyCode::Key4) {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[3].1.clone();
            next_screen.set(Screen::Level);
        } else if keyboard.just_pressed(KeyCode::Key5) {
            let level_list = level_list.get(&loading_state.level_list).unwrap();
            level_handle.handle = level_list.levels[4].1.clone();
            next_screen.set(Screen::Level);
        }
    }
}

