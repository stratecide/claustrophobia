use bevy::prelude::*;

pub fn startup(
    mut commands: Commands,
) {

    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 1000. - 0.1),
        ..Default::default()
    };
    camera.transform.scale.x = 0.5;
    camera.transform.scale.y = 0.5;
    commands.spawn(camera);
}

