use bevy::prelude::*;

use crate::game::medicine::resource::SideEffects;
use crate::game::physics::component::*;
use crate::level_loader::Level;
use crate::resource::LevelHandle;

use super::component::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    commands.spawn((
        Player,
        GravityBody,
        Movement::default(),
        DefaultCollider::default(),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(level_data.player_pos.x * side_effects.total_squish_factor(), level_data.size.y as f32 * 16. - 16. - level_data.player_pos.y, 100.),
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

