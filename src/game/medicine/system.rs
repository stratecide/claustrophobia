use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::game::physics::are_bodies_colliding;
use crate::game::physics::component::*;
use crate::game::physics::is_body_in_map_tile;
use crate::game::player::component::*;
use crate::level_loader::Level;
use crate::resource::LevelHandle;

use super::component::*;
use super::resource::*;

pub fn init_side_effects(
    side_effects: ResMut<SideEffects>,
) {
    *side_effects.into_inner() = SideEffects::new();
}

pub fn spawn_medicine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    for pill_pos in &level_data.pills {
        commands.spawn((
            Medicine::Calm,
            SpriteBundle {
                texture: asset_server.load("pill.png"),
                transform: Transform::from_xyz(pill_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - pill_pos.y, 0.),
                ..Default::default()
            },
        ));
    }
    for pill_pos in &level_data.pills2 {
        commands.spawn((
            Medicine::Cleanse,
            SpriteBundle {
                texture: asset_server.load("pill2.png"),
                transform: Transform::from_xyz(pill_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - pill_pos.y, 0.),
                ..Default::default()
            },
        ));
    }
}

pub fn spawn_couch(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    for couch_pos in &level_data.couches {
        commands.spawn((
            Couch,
            CollisionBody(Rect {
                min: Vec2::new(-16., -8.),
                max: Vec2::new(16., 0.),
            }),
            SpriteBundle {
                texture: asset_server.load("couch.png"),
                transform: Transform::from_xyz(couch_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - couch_pos.y, 2.),
                ..Default::default()
            },
        ));
    }
}

pub fn collect_medicine(
    mut commands: Commands,
    medicine_query: Query<(Entity, &Medicine, &Transform)>,
    player_query: Query<(&Transform, &CollisionBody), With<Player>>,
    mut side_effects: ResMut<SideEffects>,
) {
    for (player_pos, player_body) in &player_query {
        for (medicine_entity, medicine_type, medicine_pos) in &medicine_query {
            let relative_pos = medicine_pos.translation - player_pos.translation;
            if relative_pos.x >= player_body.0.min.x && relative_pos.x < player_body.0.max.x
            && relative_pos.y >= player_body.0.min.y && relative_pos.y < player_body.0.max.y {
                // first get rid of the medicine
                match medicine_type {
                    Medicine::Calm => {
                        if !side_effects.sedated {
                            commands.entity(medicine_entity).despawn();
                            side_effects.sedated = true;
                            side_effects.start_squish_timer(SquishDirection::Expand);
                        }
                    }
                    Medicine::Cleanse => {
                        if side_effects.sedated {
                            commands.entity(medicine_entity).despawn();
                            side_effects.sedated = false;
                            side_effects.start_squish_timer(SquishDirection::Shrink);
                        }
                    }
                }
            }
        }
    }
}

pub fn tick_side_effects(
    mut side_effects: ResMut<SideEffects>,
    time: Res<Time>,
    mut transform_query: Query<&mut Transform>,
) {
    side_effects.tick(time.delta());
    if side_effects.squish_factor() != 1. {
        for mut transform in transform_query.iter_mut() {
            transform.translation.x *= side_effects.squish_factor();
            if !side_effects.is_active() {
                // avoid floating-point issues
                // i really should have scaled the camera instead of moving entities
                transform.translation.x = transform.translation.x.round();
            }
        }
    }
}

/**
 * Checks if an entity got stuck in a wall due to squishing-effect and tries to free them
 */
pub fn fix_squished_collision_bodies(
    side_effects: Res<SideEffects>,
    mut collision_body_query: Query<(&mut Transform, &CollisionBody), With<Movement>>,
    tile_storage_query: Query<(&TileStorage, &Transform, &TilemapGridSize), Without<Movement>>,
) {
    let (tile_storage, tilemap_transform, grid_size) = match tile_storage_query.get_single() {
        Ok(ts) => ts,
        _ => return,
    };
    let grid_size: Vec2 = grid_size.into();
    let grid_size = grid_size * tilemap_transform.scale.xy();
    let tilemap_pos = tilemap_transform.translation.xy() - grid_size / 2.;

    if side_effects.squish_factor() != 1. || true {
        for (mut transform, body) in collision_body_query.iter_mut() {
            let mut pos = transform.translation.xy();
            if is_body_in_map_tile(tile_storage, &grid_size.into(), pos - tilemap_pos, body) {
                pos.x = pos.x.round();
                let offset = Vec2::new(1., 0.);
                for i in 1..17 {
                    if !is_body_in_map_tile(tile_storage, &grid_size.into(), pos - tilemap_pos + offset * i as f32, body) {
                        transform.translation.x = pos.x + i as f32;
                        break;
                    }
                    if !is_body_in_map_tile(tile_storage, &grid_size.into(), pos - tilemap_pos + offset * -i as f32, body) {
                        transform.translation.x = pos.x + -i as f32;
                        break;
                    }
                }
            }
        }
    }
}

pub fn rest_on_couch(
    mut side_effects: ResMut<SideEffects>,
    player_query: Query<(&Transform, &CollisionBody), With<Player>>,
    couch_query: Query<(&Transform, &CollisionBody), With<Couch>>,
) {
    if side_effects.total_squish_factor() != 0.5 {
        for (player_transform, player_body) in &player_query {
            let mut below_player = player_transform.translation.xy();
            below_player.y -= 1.;
            for (couch_transform, couch_body) in &couch_query {
                if are_bodies_colliding(below_player, player_body, couch_transform.translation.xy(), couch_body)
                    && player_body.0.min.x + below_player.x >= couch_body.0.min.x + couch_transform.translation.x
                    && player_body.0.max.x + below_player.x <= couch_body.0.max.x + couch_transform.translation.x {

                    side_effects.sedated = false;
                    side_effects.start_squish_timer(SquishDirection::Shrink);
                }
            }
        }
    }
}

