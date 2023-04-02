use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::from_array([0., -2000.]))
    }
}

