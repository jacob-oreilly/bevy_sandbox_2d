use bevy::prelude::*;
use systems::*;

mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player))
        .run()
}
