use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::game::medicine::resource::SideEffects;
use crate::game::physics::component::*;
use crate::game::player::component::Player;
use crate::resource::LevelHandle;
use crate::level_loader::Level;

use super::component::*;
use super::*;

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    for enemy_pos in &level_data.patrol_spawners {
        commands.spawn((
            PatrolSpawner(None),
            SpriteBundle {
                texture: asset_server.load("enemy/patrol_spawner.png"),
                transform: Transform::from_xyz(enemy_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - enemy_pos.y, 1.),
                ..Default::default()
            },
        ));
    }
    for enemy_pos in &level_data.patrol_despawners {
        println!("added despawner");
        commands.spawn((
            PatrolDespawner,
            SpriteBundle {
                texture: asset_server.load("enemy/patrol_despawner.png"),
                transform: Transform::from_xyz(enemy_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - enemy_pos.y, 10.),
                ..Default::default()
            },
        ));
    }
    for enemy_pos in &level_data.patrols {
        commands.spawn((
            Patrol(false),
            GravityBody,
            Movement {
                speed: Vec2::new(PATROL_SPEED_NORMAL, 0.),
                ..Default::default()
            },
            HitBox(Rect {
                min: Vec2::new(-9., -16.),
                max: Vec2::new(9., 1.),
            }),
            DefaultCollider {
                body: CollisionBody(Rect {
                    min: Vec2::new(-8., -16.),
                    max: Vec2::new(8., 0.),
                }),
                ..Default::default()
            },
            SemiSolid {
                top_left: Vec2::new(-8., 0.),
                width: 16.,
            },
            SpriteBundle {
                texture: asset_server.load("enemy/patrol_angry.png"),
                transform: Transform::from_xyz(enemy_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - enemy_pos.y, 0.),
                ..Default::default()
            },
        ));
    }
}

pub fn control_enemy(
    mut commands: Commands,
    mut patrol_list: Query<(Entity, &mut Movement, &mut Transform, &mut Handle<Image>, &mut Patrol)>,
    side_effects: Res<SideEffects>,
    asset_server: Res<AssetServer>,
) {
    let (patrol_speed, texture) = if side_effects.sedated {
        (PATROL_SPEED_CALM, asset_server.load("enemy/patrol_happy.png"))
    } else {
        (PATROL_SPEED_NORMAL, asset_server.load("enemy/patrol_angry.png"))
    };
    for (entity, mut movement, mut transform, mut image, mut patrol) in patrol_list.iter_mut() {
        if movement.speed.x == 0. {
            transform.scale.x *= -1.;
        }
        movement.speed.x = patrol_speed * transform.scale.x;
        if !side_effects.is_active() && *image != texture {
            *image = texture.clone();
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                if side_effects.sedated {
                    entity_commands.remove::<HitBox>();
                } else {
                    entity_commands.insert(HitBox(Rect {
                        min: Vec2::new(-9., -16.),
                        max: Vec2::new(9., 1.),
                    }));
                }
            }
        }
        if !side_effects.is_active() && !side_effects.sedated && !movement.grounded && patrol.0 {
            // went over an edge, jump
            movement.speed.y = 300.;
        }
        patrol.0 = movement.grounded;
    }
}

pub fn enemy_kills_player(
    mut commands: Commands,
    hitbox_query: Query<(&Transform, &HitBox)>,
    player_query: Query<(Entity, &Transform, &CollisionBody), With<Player>>,
) {
    for (player_entity, player_transform, player) in &player_query {
        for (enemy_transform, hitbox) in &hitbox_query {
            if player_transform.translation.x + player.0.min.x < enemy_transform.translation.x + hitbox.0.max.x
            && player_transform.translation.x + player.0.max.x > enemy_transform.translation.x + hitbox.0.min.x
            && player_transform.translation.y + player.0.min.y < enemy_transform.translation.y + hitbox.0.max.y
            && player_transform.translation.y + player.0.max.y > enemy_transform.translation.y + hitbox.0.min.y {
                if let Some(mut player_commands) = commands.get_entity(player_entity) {
                    player_commands.despawn();
                }
            }
        }
    }
}

pub fn spawn_patrol(
    mut commands: Commands,
    mut spawner_query: Query<(&mut PatrolSpawner, &Transform)>,
    patrol_query: Query<Entity, With<Patrol>>,
    asset_server: Res<AssetServer>,
    side_effects: Res<SideEffects>,
) {
    let mut existing_patrols = HashSet::new();
    for patrol in &patrol_query {
        existing_patrols.insert(patrol);
    }
    for (mut spawner, transform) in spawner_query.iter_mut() {
        if spawner.0.and_then(|e| Some(!existing_patrols.contains(&e))).unwrap_or(true) {
            let mut cmd = commands.spawn((
                Patrol(false),
                GravityBody,
                Movement {
                    speed: Vec2::new(PATROL_SPEED_NORMAL, 0.),
                    ..Default::default()
                },
                HitBox(Rect {
                    min: Vec2::new(-9., -16.),
                    max: Vec2::new(9., 1.),
                }),
                DefaultCollider {
                    body: CollisionBody(Rect {
                        min: Vec2::new(-8., -16.),
                        max: Vec2::new(8., 0.),
                    }),
                    ..Default::default()
                },
                SemiSolid {
                    top_left: Vec2::new(-8., 0.),
                    width: 16.,
                },
                SpriteBundle {
                    texture: asset_server.load("enemy/patrol_angry.png"),
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y + 8., 0.),
                    ..Default::default()
                },
            ));
            if side_effects.sedated {
                cmd.remove::<HitBox>();
            }
            spawner.0 = Some(cmd.id());
        }
    }
}

pub fn despawn_patrol(
    mut commands: Commands,
    despawner_query: Query<&Transform, With<PatrolDespawner>>,
    patrol_query: Query<(Entity, &Transform), With<Patrol>>,
) {
    for transform in despawner_query.iter() {
        for (entity, enemy_transform) in &patrol_query {
            if (enemy_transform.translation.x - transform.translation.x).abs() < 8.
            && (enemy_transform.translation.y - 24. - transform.translation.y).abs() < 8. {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.despawn();
                }
            }
        }
    }
}

