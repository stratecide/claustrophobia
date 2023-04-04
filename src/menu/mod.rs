mod system;

use bevy::prelude::*;
use system::*;
use crate::game::physics::PhysicsSystemSet;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(transition_to_level.before(PhysicsSystemSet::SetMovementThisFrame))
            ;
    }
}

