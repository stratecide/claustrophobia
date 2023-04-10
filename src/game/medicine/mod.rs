pub mod component;
mod system;
pub mod resource;

use bevy::prelude::*;
use system::*;

use crate::resource::Screen;
use self::resource::SideEffects;

use super::physics::PhysicsSystemSet;

pub struct MedicinePlugin;

impl Plugin for MedicinePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SideEffects::new())
            .add_systems((
//                    init_side_effects,
                    spawn_medicine,
                ).in_schedule(OnEnter(Screen::Level))
            )
            .add_system(collect_medicine.in_set(PhysicsSystemSet::Collisions))
            .add_system(tick_side_effects)
            .add_system(fix_squished_collision_bodies.after(tick_side_effects))
            ;
    }
}


