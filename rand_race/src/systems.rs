use super::components::Player;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub const PLAYER_SPEED: f32 = 500.0;

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

pub fn spawn_entities(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(Color::rgba_linear(255.0, 255.0, 255.0, 1.0).into()),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player {},
    ));
}

//function handles the cars driving.
//TODO need to add coasting on button release and calculating real physics.
pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            println!("Drive forward");
        }
        if keys.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            println!("Turn player left");
        }
        if keys.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            println!("Turn player right");
        }
        if keys.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            println!("Reverse");
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

//Implement camera follow and turn with car so forward direction won't always be the same.
