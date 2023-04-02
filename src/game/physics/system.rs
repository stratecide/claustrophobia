use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_ecs_tilemap::prelude::*;

use super::component::*;
use super::resource::*;
use super::is_body_in_map_tile;

pub fn set_speed_this_frame(
    mut movement_query: Query<(&mut Movement, &Transform)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut movement, transform) in movement_query.iter_mut() {
        movement.next_pos = transform.translation.xy() + movement.speed * delta_seconds;
    }
}

pub fn map_collision(
    mut body_query: Query<(&mut Movement, &Transform, &CollisionBody), With<MapCollider>>,
    tile_storage_query: Query<(&TileStorage, &Transform, &TilemapGridSize)>,
) {
    let (tile_storage, tilemap_transform, grid_size) = match tile_storage_query.get_single() {
        Ok(ts) => ts,
        _ => return,
    };

    for (mut movement, transform, body) in body_query.iter_mut() {
        let mut relative_translation = transform.translation.xy() - tilemap_transform.translation.xy();
        let this_frame = movement.next_pos - transform.translation.xy();

        // check horizontal movement first so the player has an easier time landing on platforms
        let steps = this_frame.x.abs().ceil();
        for x in 1..steps as u32 + 1 {
            let movement_this_step = Vec2 {
                x: x as f32 / steps * this_frame.x,
                y: 0.,
            };
            let position_to_check = relative_translation + movement_this_step;
            if is_body_in_map_tile(tile_storage, grid_size, position_to_check, body) {
                if this_frame.x > 0. {
                    movement.next_pos.x = position_to_check.x.ceil() - 1. + tilemap_transform.translation.x;
                } else {
                    movement.next_pos.x = position_to_check.x.floor() + 1. + tilemap_transform.translation.x;
                }
                movement.speed.x = 0.;
                break;
            }
        }

        // reset grounded
        movement.grounded = false;

        // don't get stuck in corners
        relative_translation.x = movement.next_pos.x - tilemap_transform.translation.x;

        // vertical movement
        let steps = this_frame.y.abs().ceil();
        for y in 1..steps as u32 + 1 {
            let movement_this_step = Vec2 {
                x: 0.,
                y: y as f32 / steps * this_frame.y,
            };
            let position_to_check = relative_translation + movement_this_step;
            if is_body_in_map_tile(tile_storage, grid_size, position_to_check, body) {
                if this_frame.y > 0. {
                    movement.next_pos.y = position_to_check.y.ceil() - 1. + tilemap_transform.translation.y;
                } else {
                    movement.next_pos.y = position_to_check.y.floor() + 1. + tilemap_transform.translation.y;
                    movement.grounded = true;
                }
                movement.speed.y = 0.;
                break;
            }
        }
    }
}

pub fn movement_step(
    mut body_query: Query<(&mut Transform, &Movement)>,
) {
    for (mut transform, movement) in body_query.iter_mut() {
        transform.translation = movement.next_pos.extend(0.);
    }
}

pub fn apply_gravity(
    mut movement_query: Query<&mut Movement, With<GravityBody>>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    let speed_change = gravity.0 * time.delta_seconds();
    for mut movement in movement_query.iter_mut() {
        movement.speed += speed_change;
    }
}

