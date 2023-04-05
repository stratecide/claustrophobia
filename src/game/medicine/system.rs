use bevy::prelude::*;

use crate::game::physics::component::CollisionBody;
use crate::game::player::component::*;
use crate::level_loader::Level;
use crate::resource::LevelHandle;

use super::component::Medicine;
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
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    for pill_pos in &level_data.pills {
        commands.spawn((
            Medicine,
            SpriteBundle {
                texture: asset_server.load("pill.png"),
                transform: Transform::from_xyz(pill_pos.x, level_data.size.y as f32 * 16. - 16. - pill_pos.y, 0.),
                ..Default::default()
            },
        ));
        println!("spawned pill at {}, {}", pill_pos.x, level_data.size.y as f32 * 16. - 16. - pill_pos.y);
    }

    println!("spawned {} pills", level_data.pills.len());
}

pub fn collect_medicine(
    mut commands: Commands,
    medicine_query: Query<(Entity, &Medicine, &Transform)>,
    player_query: Query<(&Transform, &CollisionBody), With<Player>>,
    mut side_effects: ResMut<SideEffects>,
) {
    for (player_pos, player_body) in &player_query {
        for (medicine_entity, _medicine_type, medicine_pos) in &medicine_query {
            let relative_pos = medicine_pos.translation - player_pos.translation;
            if relative_pos.x >= player_body.0.min.x && relative_pos.x < player_body.0.max.x
                && relative_pos.y >= player_body.0.min.y && relative_pos.y < player_body.0.max.y {
                // first get rid of the medicine
                commands.entity(medicine_entity).despawn();
                side_effects.sedated = true;
                side_effects.start_squish_timer(SquishDirection::Expand);
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
        }
    }
}

