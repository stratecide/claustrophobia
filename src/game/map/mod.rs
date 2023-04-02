mod system;

use system::*;

use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup)
            .add_startup_system(fix_tilemap_edges.after(startup))
            ;
    }
}

