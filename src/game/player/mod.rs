pub mod component;
mod system;

use bevy::prelude::*;

use system::*;

use super::physics::PhysicsSystemSet;

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_set(InputSystemSet.before(PhysicsSystemSet::SetMovementThisFrame))
            .add_startup_system(spawn_player)
            .add_system(player_input)
            ;
    }
}

