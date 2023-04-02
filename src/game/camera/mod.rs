mod system;

use bevy::prelude::*;

use system::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_startup_system(startup)
            ;
    }
}

