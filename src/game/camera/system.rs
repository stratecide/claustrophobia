use bevy::prelude::*;

use crate::resource::LevelHandle;
use crate::level_loader::Level;

pub fn startup(
    mut commands: Commands,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(level_data.size.x as f32 * 8., level_data.size.y as f32 * 8., 1000. - 0.1),
        ..Default::default()
    };
    camera.transform.scale.x = 0.5;
    camera.transform.scale.y = 0.5;
    commands.spawn(camera);
}

