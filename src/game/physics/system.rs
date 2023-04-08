use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_ecs_tilemap::prelude::*;

use crate::game::medicine::resource::SideEffects;

use super::component::*;
use super::resource::*;
use super::*;

pub fn set_speed_this_frame(
    mut movement_query: Query<(&mut Movement, &Transform)>,
    time: Res<Time>,
    side_effects: Res<SideEffects>,
) {
    let delta_seconds = time.delta_seconds();

    let paused = side_effects.is_active();

    for (mut movement, transform) in movement_query.iter_mut() {
        movement.next_pos = transform.translation.xy();
        if !paused {
            let next_pos = movement.next_pos;
            movement.next_pos = next_pos + movement.speed * delta_seconds;
            movement.grounded = false;
        }
    }
}

pub fn handle_collisions(
    mut body_query: Query<(&mut Movement, &Transform, &CollisionBody), With<MapCollider>>,
    tile_storage_query: Query<(&TileStorage, &Transform, &TilemapGridSize)>,
    static_body_query: Query<(&Transform, &CollisionBody), Without<Movement>>,
    semi_solid_query: Query<(&Transform, &SemiSolid)>,
) {
    let (tile_storage, tilemap_transform, grid_size) = match tile_storage_query.get_single() {
        Ok(ts) => ts,
        _ => return,
    };
    let grid_size: Vec2 = grid_size.into();
    let grid_size = grid_size * tilemap_transform.scale.xy();
    let tilemap_translation = tilemap_transform.translation.xy() - grid_size / 2.;

    for (mut movement, transform, body) in body_query.iter_mut() {
        let mut relative_translation: Vec2 = transform.translation.xy() - tilemap_translation;
        let this_frame = movement.next_pos - transform.translation.xy();

        // check horizontal movement first so the player has an easier time landing on platforms
        let steps = this_frame.x.abs().ceil();
        for x in 1..steps as u32 + 1 {
            let movement_this_step = Vec2 {
                x: x as f32 / steps * this_frame.x,
                y: 0.,
            };
            let position_to_check = relative_translation + movement_this_step;
            if is_body_in_map_tile(tile_storage, &grid_size.into(), position_to_check, body) {
                if this_frame.x > 0. {
                    movement.next_pos.x = position_to_check.x.ceil() - 1. + tilemap_translation.x;
                } else {
                    movement.next_pos.x = position_to_check.x.floor() + 1. + tilemap_translation.x;
                }
                movement.speed.x = 0.;
                break;
            }
            let mut should_break = false;
            for (static_transform, static_body) in &static_body_query {
                if are_bodies_colliding(transform.translation.xy() + movement_this_step, body, static_transform.translation.xy(), static_body) {
                    if this_frame.x > 0. {
                        movement.next_pos.x = static_transform.translation.x + static_body.0.min.x - body.0.max.x;
                    } else {
                        movement.next_pos.x = static_transform.translation.x + static_body.0.max.x - body.0.min.x;
                    }
                    movement.speed.x = 0.;
                    should_break = true;
                }
            }
            if should_break {
                break;
            }
        }

        // don't get stuck in corners
        relative_translation.x = movement.next_pos.x - tilemap_translation.x;

        // vertical movement
        let steps = this_frame.y.abs().ceil();
        for y in 1..steps as u32 + 1 {
            let next_pos = Vec2 {
                x: movement.next_pos.x,
                y: transform.translation.y + y as f32 / steps * this_frame.y,
            };
            if is_body_in_map_tile(tile_storage, &grid_size.into(), next_pos - tilemap_translation, body) {
                if this_frame.y > 0. {
                    movement.next_pos.y = next_pos.y.ceil() - 1.;
                } else {
                    movement.next_pos.y = next_pos.y.floor() + 1.;
                    movement.grounded = true;
                }
                movement.speed.y = 0.;
                break;
            }
            let mut should_break = false;
            for (static_transform, static_body) in &static_body_query {
                if are_bodies_colliding(next_pos, body, static_transform.translation.xy(), static_body) {
                    if this_frame.y > 0. {
                        movement.next_pos.y = static_transform.translation.y + static_body.0.min.y - body.0.max.y;
                    } else {
                        movement.next_pos.y = static_transform.translation.y + static_body.0.max.y - body.0.min.y;
                        movement.grounded = true;
                    }
                    movement.speed.y = 0.;
                    should_break = true;
                }
            }
            if !should_break && this_frame.y < 0. {
                for (ss_transform, semi_solid) in &semi_solid_query {
                    if ss_transform.translation.x + semi_solid.top_left.x < movement.next_pos.x + body.0.max.x
                    && ss_transform.translation.x + semi_solid.top_left.x + semi_solid.width > movement.next_pos.x + body.0.min.x
                    && ss_transform.translation.y + semi_solid.top_left.y <= transform.translation.y + body.0.min.y
                    && ss_transform.translation.y + semi_solid.top_left.y > movement.next_pos.y + body.0.min.y {
                        movement.next_pos.y = ss_transform.translation.y + semi_solid.top_left.y - body.0.min.y;
                        movement.grounded = true;
                        movement.speed.y = 0.;
                        should_break = true;
                    }
                }
            }
            if should_break {
                break;
            }
        }
    }
}

pub fn movement_step(
    mut body_query: Query<(&mut Transform, &Movement)>,
) {
    for (mut transform, movement) in body_query.iter_mut() {
        if movement.next_pos.length() == 0. {
            // TODO: fix this properly
            continue;
        }
        transform.translation = movement.next_pos.extend(transform.translation.z);
    }
}

pub fn apply_gravity(
    mut movement_query: Query<&mut Movement, With<GravityBody>>,
    time: Res<Time>,
    gravity: Res<Gravity>,
    side_effects: Res<SideEffects>,
) {
    if side_effects.is_active() {
        return;
    }
    let speed_change = gravity.0 * time.delta_seconds();
    for mut movement in movement_query.iter_mut() {
        movement.speed += speed_change;
    }
}

