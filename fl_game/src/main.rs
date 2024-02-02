use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player, spawn_tourch))
        .add_systems(Update, player_movement)
        .run()
}
