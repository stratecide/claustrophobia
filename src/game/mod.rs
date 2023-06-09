pub mod camera;
pub mod enemy;
pub mod map;
pub mod medicine;
pub mod player;
pub mod physics;

pub mod component;
mod system;

use bevy::prelude::*;

use camera::*;
use enemy::*;
use map::*;
use medicine::*;
use player::*;
use physics::*;
use system::*;

use crate::resource::Screen;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(MedicinePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            .add_system(next_level.in_set(OnUpdate(Screen::NextLevel)))
            ;
    }
}


