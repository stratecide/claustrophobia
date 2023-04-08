mod system;

use bevy::prelude::*;
use system::*;
use crate::resource::Screen;

use super::physics::PhysicsSystemSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(startup.in_schedule(OnEnter(Screen::Level)))
            .add_system(follow_player.in_set(PhysicsSystemSet::AfterMovement))
            ;
    }
}

