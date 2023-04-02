use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod game;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(GamePlugin)
        .run();
}
