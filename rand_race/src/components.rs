use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub(crate) movement_speed: f32,
    pub(crate) rotation_speed: f32,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub(crate) focus: Vec3,
}

#[derive(Component)]
pub struct RoadBlock {
    pub(crate) location: Vec3,
    pub(crate) rotation: f32,
    pub(crate) tile_ref: str
}