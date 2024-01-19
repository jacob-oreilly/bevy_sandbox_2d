use bevy::{prelude::*, sprite::{Mesh2d, MaterialMesh2dBundle}, window::PrimaryWindow, transform};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    window_query: Query<&Window, With<PrimaryWindow>>) {
        let window = window_query.get_single().unwrap();
        commands.spawn(Camera2dBundle::default());
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
            .add(shape::Quad::new(Vec2::new(2.0, 300.)).into())
            .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.)),
            ..default()
        });
    println!("Ray Casting");
}