use bevy::prelude::*;

use crate::{resource::*, level_loader::LevelList};

pub fn setup(
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<LoadingState>,
) {
    loading_state.level_list = asset_server.load("levels/claustrophobia.ldtk");
}

pub fn loading_progress(
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
    loading_state: Res<LoadingState>,
    level_list_assets: Res<Assets<LevelList>>,
) {
    match screen.0 {
        Screen::Loading => {
            let level_list = level_list_assets.get(&loading_state.level_list);
            if level_list.is_some() {
                println!("levels finished loading");
                next_screen.set(Screen::MainMenu);
            }
        }
        _ => ()
    }
}

