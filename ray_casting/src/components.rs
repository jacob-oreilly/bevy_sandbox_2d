use bevy::prelude::*;

#[derive(Component)]
pub struct Ray {}

#[derive(Component)]
pub struct Wall {
    pub point_a: Vec3,
    pub point_b: Vec3,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct RayDirection {
    pub direction_x: f32,
    pub direction_y: f32
}