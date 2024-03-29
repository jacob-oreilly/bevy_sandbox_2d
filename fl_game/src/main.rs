use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (setup, spawn_player, spawn_walls))
        .add_systems(FixedUpdate, (player_movement, tourch_light_update, update_tourch_light_direction))
        // .add_systems(Update, tourch_light_update.after(player_movement))
        .run()
}
