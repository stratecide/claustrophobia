mod system;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::sprite::*;
use system::*;
use crate::resource::Screen;

use super::physics::PhysicsSystemSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<ScarySidesMaterial>::default())
            .add_system(startup.in_schedule(OnEnter(Screen::Level)))
            .add_system(follow_player.in_set(PhysicsSystemSet::AfterMovement))
            .add_system(sides_follow_player.after(follow_player))
            .add_system(set_window_size)
            ;
    }
}

#[derive(Debug, Clone, TypeUuid, AsBindGroup)]
#[uuid = "b1ef3384-8173-4f07-b3af-fa6cf393f67a"]
pub struct ScarySidesMaterial {}

impl Material2d for ScarySidesMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shader.wgsl".into()
    }
}

