use bevy::prelude::*;

#[derive(Component, Default)]
pub struct HitBox(pub Rect);

#[derive(Component, Default)]
pub struct Patrol(pub bool);

