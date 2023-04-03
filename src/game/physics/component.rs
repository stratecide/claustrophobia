use bevy::prelude::*;

#[derive(Component)]
pub struct GravityBody;

#[derive(Component, Default)]
pub struct Movement {
    pub speed: Vec2,
    pub next_pos: Vec2,
    pub grounded: bool,
}

#[derive(Component)]
pub struct CollisionBody(pub Rect);
impl Default for CollisionBody {
    fn default() -> Self {
        Self(Rect {
            min: Vec2::from_array([-8., -8.]),
            max: Vec2::from_array([8., 8.]),
        })
    }
}

#[derive(Component, Default)]
pub struct MapCollider;

#[derive(Bundle, Default)]
pub struct DefaultCollider {
    pub body: CollisionBody,
    pub map_collider: MapCollider,
}

