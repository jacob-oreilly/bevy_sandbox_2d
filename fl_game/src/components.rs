use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
}

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub player_rotation: f32,
}

#[derive(Component)]
pub struct Tourch {

}

#[derive(Component)]
pub struct TourchLight {
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub ray_direction: Vec2,
}

#[derive(Component)]
pub struct Wall {
    pub start: Vec3,
    pub end: Vec3
}

#[derive(Component)]
pub struct PlayerParent {
}