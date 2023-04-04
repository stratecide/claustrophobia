use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod game;
pub mod menu;

mod level_loader;
pub mod resource;
mod system;

use game::GamePlugin;
use menu::MenuPlugin;

use level_loader::*;
use resource::*;
use system::*;

fn main() {
    App::new()
        .add_state::<Screen>()

        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)

        .add_asset::<LevelList>()
        .add_asset::<Level>()
        .init_asset_loader::<LevelLoader>()
        .init_resource::<LoadingState>()
        .init_resource::<LevelHandle>()
        .add_startup_system(setup)
        .add_system(loading_progress)

        .run();
}
