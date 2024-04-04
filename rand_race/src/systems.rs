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
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(Rectangle::new(10.0, 20.0))),
            material: materials.add(Color::rgba_linear(255.0, 255.0, 255.0, 1.0)),
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
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {

    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = 0.0;
        let mut rotation = 0.0;
        if keys.pressed(KeyCode::KeyW) {
            direction += 1.0;
            println!("Drive forward");
        }
        if keys.pressed(KeyCode::KeyA) {
            rotation += 1.0;
            println!("Turn player left");
        }
        if keys.pressed(KeyCode::KeyD) {
            rotation -= 1.0;
            println!("Turn player right");
        }
        if keys.pressed(KeyCode::KeyS) {
            direction -= 1.0;
            println!("Reverse");
        }

        transform.rotate_z(rotation * player.rotation_speed * time.delta_seconds());
        let transform_rotate = transform.rotation;
        let movement_direction = transform_rotate * Vec3::Y;
        let movement_distance = direction * player.movement_speed * time.delta_seconds();
        transform.translation += movement_direction * movement_distance;
        // println!("Player loc: {:?}", transform.translation);
       
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

pub fn generate_road(mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>) {
        let window = window_query.get_single().unwrap();
        let starting_point = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
        // let mut map_vec:Vec<i64> = Vec::new();
        // commands.spawn(SpriteBundle{
        //     ..default()
        // });
        // for i in 0..track_length {
        //     let mut rng = rand::thread_rng();
        //     let rand_point: i64 = rng.gen_range(0..=9);
        //     map_vec.push(rand_point);
        // }
    
}

// fn read_csv_map() -> Result<(), Box<dyn Error>> {
//     let f = std::fs::File::open("test.csv")?;
//     // let mut reader = Reader::from_path("test.csv")?;
//     let mut rdr: Reader<Box<dyn io::Read>> = csv::ReaderBuilder::new().delimiter(b',').from_reader(Box::new(f));
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     };

//     Ok(())
// }