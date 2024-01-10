use bevy::{prelude::*, window::PrimaryWindow, sprite::MaterialMesh2dBundle};
use super::components::Player;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn setup() {
    println!("Random Race!");
}

pub fn spawn_entities(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>, window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(
                    Color::rgba_linear(
                        255.0,
                        255.0,
                        255.0,
                        1.0,
                    )
                    .into(),
                ),
                transform: Transform::from_xyz(window.width() / 2.0 , window.height() / 2.0, 0.0),
                ..default()
            },
            Player {},
        )
    );
}