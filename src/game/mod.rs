pub mod camera;
pub mod player;
pub mod enemy;
pub mod map;
pub mod medicine;
pub mod physics;

mod system;

use bevy::prelude::*;

use camera::*;
use map::*;
use medicine::*;
use player::*;
use physics::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(MedicinePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            ;
    }
}


