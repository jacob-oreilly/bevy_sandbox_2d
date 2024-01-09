use bevy::prelude::*;

mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_camera))
        .run();
}

fn setup() {
    println!("Random Race!");
}
