pub mod component;
mod system;

use bevy::prelude::*;
use system::*;

use crate::resource::Screen;

use super::physics::PhysicsSystemSet;

pub const PATROL_SPEED_NORMAL: f32 = 50.;
pub const PATROL_SPEED_CALM: f32 = 20.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemy.in_schedule(OnEnter(Screen::Level)))
            .add_system(control_enemy.before(PhysicsSystemSet::SetMovementThisFrame))
            ;
    }
}

