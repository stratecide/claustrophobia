pub mod component;
mod system;

use bevy::prelude::*;
use system::*;
use crate::resource::Screen;

use super::physics::PhysicsSystemSet;

pub struct MedicinePlugin;

impl Plugin for MedicinePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_medicine.in_schedule(OnEnter(Screen::Level)))
            .add_system(collect_medicine.in_set(PhysicsSystemSet::Collisions))
            ;
    }
}


