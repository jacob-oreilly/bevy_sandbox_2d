use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_camera, spawn_entities))
        .add_systems(Update, (player_movement, update_camera))
        .run();
}
