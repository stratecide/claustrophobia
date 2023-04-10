use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::game::component::GameElement;
use crate::game::medicine::resource::SideEffects;
use crate::game::physics::are_bodies_colliding;
use crate::game::physics::component::*;
use crate::level_loader::Level;
use crate::resource::LevelHandle;
use crate::resource::Screen;

use super::component::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();
    if level_data.player_pos.x == 0. && level_data.player_pos.y == 0. {
        return;
    }

    commands.spawn((
        GameElement,
        Player,
        GravityBody,
        SemiSolid {
            top_left: Vec2::new(-8., 8.),
            width: 16.,
        },
        Movement::default(),
        DefaultCollider::default(),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(level_data.player_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - level_data.player_pos.y, 10.),
            ..Default::default()
        },
    ));

    println!("spawned player at {}, {}", level_data.player_pos.x, level_data.size.y as f32 * 16. - 16. - level_data.player_pos.y);
}

pub fn player_input(
    mut player_query: Query<&mut Movement, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    side_effects: Res<SideEffects>,
) {
    let mut horizontal = 0.;
    let pressed_jump = keyboard.just_pressed(KeyCode::Up) || keyboard.just_pressed(KeyCode::W);
    if keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D) {
        horizontal += 1.;
    }
    if keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A) {
        horizontal -= 1.;
    }

    let delta_seconds = time.delta_seconds();
    let (hspeed, jump_strength) = if side_effects.sedated {
        (1000., 400.)
    } else {
        (1500., 500.)
    };

    for mut player in player_query.iter_mut() {
        player.speed.x *= 0.0005_f32.powf(delta_seconds);
        player.speed.x += horizontal * hspeed * delta_seconds;
        if horizontal == 0. && player.speed.x.abs() < 2. {
            player.speed.x = 0.;
        }
        if pressed_jump && player.grounded {
            player.speed.y = jump_strength;
        }
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
            GameElement,
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

pub fn rest_on_couch(
    mut commands: Commands,
    player_query: Query<(&Transform, &CollisionBody), With<Player>>,
    couch_query: Query<(&Transform, &CollisionBody), With<Couch>>,
    entity_query: Query<Entity, With<GameElement>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut level_handle: ResMut<LevelHandle>,
) {
    for (player_transform, player_body) in &player_query {
        let mut below_player = player_transform.translation.xy();
        below_player.y -= 1.;
        for (couch_transform, couch_body) in &couch_query {
            if are_bodies_colliding(below_player, player_body, couch_transform.translation.xy(), couch_body)
            && player_body.0.min.x + below_player.x >= couch_body.0.min.x + couch_transform.translation.x
            && player_body.0.max.x + below_player.x <= couch_body.0.max.x + couch_transform.translation.x {
                for entity in &entity_query {
                    commands.entity(entity).despawn();
                }
                level_handle.level_id += 1;
                next_screen.set(Screen::NextLevel);
                return;
            }
        }
    }
}

pub fn reset_level(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
    entity_query: Query<Entity, With<GameElement>>,
) {
    if screen.0 == Screen::Level {
        if keyboard.just_pressed(KeyCode::R) {
            for entity in &entity_query {
                commands.entity(entity).despawn();
            }
            next_screen.set(Screen::NextLevel);
        }
    }
}

