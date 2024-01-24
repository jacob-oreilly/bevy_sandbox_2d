
use bevy::prelude::*;

mod systems;
use components::*;
use resources::{Mouse, RayAssets};
use systems::*;
pub mod components;
pub mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_rays))
        .add_systems(Update, (cursor_events, ray_intersect_update))
        .insert_resource(Mouse::default())
        .insert_resource(RayAssets::default())
        .run();
}
