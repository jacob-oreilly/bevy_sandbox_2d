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
pub struct Wall {

}

#[derive(Component)]
pub struct PlayerParent {

}