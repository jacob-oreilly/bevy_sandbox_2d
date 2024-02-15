use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_floor))
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(meshes.add(shape::Box::new(10.0, 25.0, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
                transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
                ..default()
            },
            Player{},
        )
    );
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(meshes.add(shape::Box::new(400.0, 10.0, 0.0).into())),
                material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
                transform: Transform::from_translation(Vec3::new(window.width() / 2.0, (window.height() / 2.0) - 200.0, 0.0)),
                ..default()
            },
            Player{},
        )
    );
}
