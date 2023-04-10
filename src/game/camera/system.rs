use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::WindowResolution;

use crate::game::component::GameElement;
use crate::game::medicine::resource::SideEffects;
use crate::game::player::component::Player;
use crate::resource::LevelHandle;
use crate::level_loader::Level;

use super::ScarySidesMaterial;

pub fn startup(
    mut commands: Commands,
    window_query: Query<&Window>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
    side_effects: Res<SideEffects>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ScarySidesMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    let camera_x = level_data.size.x as f32 * 8. * side_effects.total_squish_factor();
    let camera_y = level_data.size.y as f32 * 8.;
    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(camera_x, camera_y, 1000. - 0.1),
        ..Default::default()
    };
    camera.transform.scale.x = 0.5;
    camera.transform.scale.y = 0.5;
    commands.spawn((
        GameElement,
        camera,
    ));

    commands.spawn((
        GameElement,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(window.width() / 8., window.height() / 2.),
                flip: false,
            }))),
            transform: Transform::from_xyz(0., 0., 20.),
            material: materials.add(ScarySidesMaterial {}),
            ..default()
        }
    ));
    let mut transform = Transform::from_xyz(0., 0., 20.);
    transform.scale.x = -1.;
    commands.spawn((
        GameElement,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(window.width() / 8., window.height() / 2.),
                flip: false,
            }))),
            transform,
            material: materials.add(ScarySidesMaterial {}),
            ..default()
        },
    ));
}

pub fn follow_player(
    player_query: Query<(&Transform, &Player), Without<Camera>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok((player_transform, _)) = player_query.get_single() {
        let mut camera_transform = camera_query.get_single_mut().unwrap();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn sides_follow_player(
    camera_query: Query<&Transform, With<Camera>>,
    mut sides_query: Query<(&Mesh2dHandle, &mut Transform), Without<Camera>>,
    window_query: Query<&Window>,
    side_effects: Res<SideEffects>,
) {
    if let Ok(camera) = camera_query.get_single() {
        let window = window_query.get_single().unwrap();
        for (_, mut transform) in &mut sides_query {
            let offset = window.width() / 4. * (0.25 + side_effects.total_squish_factor());
            transform.translation.x = camera.translation.x - offset * transform.scale.x;
            transform.translation.y = camera.translation.y;
        }
    }
}

pub fn set_window_size(
    mut windows: Query<&mut Window>,
) {
    for mut window in &mut windows {
        window.resolution = WindowResolution::new(960., 640.);
        window.resize_constraints.min_width = 960.;
        window.resize_constraints.max_width = 960.;
        window.resize_constraints.min_height = 640.;
        window.resize_constraints.max_height = 640.;
    }
}

