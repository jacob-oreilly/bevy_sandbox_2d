use crate::components::PlayerCamera;

use super::components::Player;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};


pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            },
            PlayerCamera {
                focus: Vec3::ZERO,
            }
        )
    );
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
            mesh: meshes.add(shape::Quad::new(Vec2 { x: 10.0, y: 20.0 }).into()).into(),
            material: materials.add(Color::rgba_linear(255.0, 255.0, 255.0, 1.0).into()),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0)
        },
    ));
}

//function handles the cars driving.
//TODO need to add coasting on button release and calculating real physics.
pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {

    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = 0.0;
        let mut rotation = 0.0;
        if keys.pressed(KeyCode::W) {
            direction += 1.0;
            println!("Drive forward");
        }
        if keys.pressed(KeyCode::A) {
            rotation += 1.0;
            println!("Turn player left");
        }
        if keys.pressed(KeyCode::D) {
            rotation -= 1.0;
            println!("Turn player right");
        }
        if keys.pressed(KeyCode::S) {
            direction -= 1.0;
            println!("Reverse");
        }

        transform.rotate_z(rotation * player.rotation_speed * time.delta_seconds());
        let transform_rotate = transform.rotation;
        let movement_direction = transform_rotate * Vec3::Y;
        let movement_distance = direction * player.movement_speed * time.delta_seconds();
        transform.translation += movement_direction * movement_distance;
        println!("Player loc: {:?}", transform.translation);
       
    }
}

//Implement camera follow and turn with car so forward direction won't always be the same.
pub fn update_camera(player_query: Query<&Transform, With<Player>>, 
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>) {
    
    let Ok(player) = player_query.get_single() else {return};
    let Ok((mut camera_transform, mut player_camera))  = camera_query.get_single_mut() else {return};
    
    let delta = player.translation - camera_transform.translation;

    if delta != Vec3::ZERO {
        player_camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}