
use std::time::Duration;

use bevy::{prelude::*, winit::WinitSettings};

mod systems;
use components::*;
use resources::{Mouse, NumberOfRays, RayAssets};
use systems::*;
pub mod components;
pub mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_rays, draw_walls))
        .add_systems(Update, (cursor_events, ray_intersect_update))
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(10),
            },
            ..default()
        })
        .insert_resource(Mouse::default())
        .insert_resource(RayAssets::default())
        .insert_resource(NumberOfRays::default())
        .run();
}
