use bevy::prelude::*;

#[derive(Component)]
pub struct Ray {
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub ray_direction: Vec2,
}

#[derive(Component)]
pub struct Wall {
    pub point_a: Vec3,
    pub point_b: Vec3,
}

#[derive(Component)]
pub struct MainCamera;