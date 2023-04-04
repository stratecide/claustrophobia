pub mod component;
mod system;

use bevy::prelude::*;

use system::*;

use crate::resource::Screen;

use super::physics::PhysicsSystemSet;

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InputSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_set(InputSystemSet.before(PhysicsSystemSet::SetMovementThisFrame))
            .add_system(spawn_player.in_schedule(OnEnter(Screen::Level)))
            .add_system(player_input.before(PhysicsSystemSet::SetMovementThisFrame))
            ;
    }
}

