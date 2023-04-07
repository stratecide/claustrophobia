pub mod component;
pub mod resource;
mod system;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use component::*;
use resource::*;
use system::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSystemSet {
    SetMovementThisFrame,
    Collisions,
    Movement,
    AfterMovement,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Gravity>()

            .configure_sets((
                    PhysicsSystemSet::SetMovementThisFrame,
                    PhysicsSystemSet::Collisions,
                    PhysicsSystemSet::Movement,
                    PhysicsSystemSet::AfterMovement,
                ).chain())

            .add_system(set_speed_this_frame.in_set(PhysicsSystemSet::SetMovementThisFrame))

            // after input is handled, check whether speeds have to be adjusted
            .add_systems((
                    handle_collisions,
                ).in_set(PhysicsSystemSet::Collisions)
            )

            // now movement can happen without problems (I hope)
            .add_system(movement_step.in_set(PhysicsSystemSet::Movement))

            // after the movement step, non-input changes to the speed of entities are handled
            .add_system(apply_gravity.in_set(PhysicsSystemSet::AfterMovement))
            ;
    }
}

// checks whether any tiles overlap the CollisionBody if it were at the given position relative to
// the tile map
pub fn is_body_in_map_tile(tile_storage: &TileStorage, grid_size: &TilemapGridSize, position: Vec2, body: &CollisionBody) -> bool {
    let min_x = ((position.x + body.0.min.x) / grid_size.x).floor().max(0.) as u32;
    let max_x = ((position.x + body.0.max.x) / grid_size.x).ceil().max(0.) as u32;
    let min_y = ((position.y + body.0.min.y) / grid_size.y).floor().max(0.) as u32;
    let max_y = ((position.y + body.0.max.y) / grid_size.y).ceil().max(0.) as u32;
    for x in min_x..max_x.min(tile_storage.size.x) {
        for y in min_y..max_y.min(tile_storage.size.y) {
            let tile_pos = TilePos { x, y };
            if tile_storage.get(&tile_pos).is_some() {
                return true;
            }
        }
    }
    false
}

pub fn are_bodies_colliding(transform1: Vec2, body1: &CollisionBody, transform2: Vec2, body2: &CollisionBody) -> bool {
    transform1.x + body1.0.min.x < transform2.x + body2.0.max.x &&
    transform1.x + body1.0.max.x > transform2.x + body2.0.min.x &&
    transform1.y + body1.0.min.y < transform2.y + body2.0.max.y &&
    transform1.y + body1.0.max.y > transform2.y + body2.0.min.y
}

/*fn is_point_in_map_tile(tile_storage: &TileStorage, grid_size: &TilemapGridSize, position: Vec2) -> bool {
    if position.x < 0. || position.y < 0. || position.x >= tile_storage.size.x as f32 * grid_size.x || position.y >= tile_storage.size.y as f32 * grid_size.y {
        return false;
    }

    let tile_pos = TilePos { x: (position.x / grid_size.x) as u32, y: (position.y / grid_size.y) as u32 };
    tile_storage.get(&tile_pos).is_some()
}*/

