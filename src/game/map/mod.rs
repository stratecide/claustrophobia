mod system;

use bevy::prelude::*;
use system::*;
use crate::resource::Screen;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(build_level.in_schedule(OnEnter(Screen::Level)))
            .add_system(fix_tilemap_edges.in_schedule(OnEnter(Screen::Level)).after(build_level))
            ;
    }
}

