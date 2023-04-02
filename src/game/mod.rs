pub mod camera;
pub mod map;
pub mod player;
pub mod enemy;
pub mod physics;

use bevy::prelude::*;

use camera::*;
use map::*;
use player::*;
use physics::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            ;
    }
}


