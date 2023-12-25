use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::color, window::PrimaryWindow};
use rand::Rng;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (600., 400.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let rez: i64 = 20;
    let cols: i64 = (600 / rez);
    let rows = (400 / rez);
    let total_col_row = cols * rows;
    let mut column_major = Vec::new();

    for _n in 1..total_col_row {
        let mut rng = rand::thread_rng();
        let rand_point: i64 = rng.gen();
        column_major.push(rand_point);
    }
    println!("{:?}", column_major); 
    for y in 1..rows {
        let y_coord = (y * rez)  as f32;
        for x in 1..cols {
            let x_coord = (x * rez) as f32;
            let current_index: usize = (y * (x - 1)).try_into().unwrap();
            let color_value = column_major[current_index] as f32;
            // Circle
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
                material: materials.add(Color::rgba_linear(color_value * 255.0, color_value * 255.0, color_value * 255.0, color_value).into()),
                transform: Transform::from_xyz(x_coord, y_coord, 0.0),
                // transform: Transform::from_translation(Vec3::new(x_coord,  y_coord, 0.)),
                ..default()
            });

            println!("cols {:?} rows {:?}", cols, rows );
            println!("point x {:?} point y {:?}", x_coord, y_coord );
            println!("computed x {:?} coomputed y {:?}", x_coord - cols as f32, y_coord - rows as f32 );
        }
        println!()
    }

    for i in 1..rows{
        for j in 1..cols {
            let y = i * rez;
            let x = j * rez;
            let point_a = pointVector {
                x_coord: x as f32 + (rez as f32 * 0.5),
                y_coord: y as f32
            };
            let point_b = pointVector {
                x_coord: (x + rez) as f32,
                y_coord: y as f32 + (rez as f32  * 0.5)
            };
            let point_c = pointVector {
                x_coord: (x + rez) as f32 * 0.5,
                y_coord: (y + rez) as f32,
            };
            let point_d = pointVector {
                x_coord: x as f32,
                y_coord: (y + rez) as f32 * 0.5
            };
            let diff_vector = Vec2::new(point_b.x_coord, point_b.y_coord) - Vec2::new(point_a.x_coord, point_a.y_coord);
            let angle = diff_vector.y.atan2(diff_vector.x);
            let abs_difference_1 = (diff_vector.y.atan2(diff_vector.x) - (-std::f32::consts::FRAC_PI_4)).abs().to_degrees();
            // let angle = diff_vector.angle_between(Vec2::new(point_a.x_coord, point_a.y_coord));
            let length_a_b = (Vec2::new(point_b.x_coord, point_b.y_coord) ).distance(Vec2::new(point_a.x_coord, point_a.y_coord));
            // let point_ab_angle = (Vec2::new(point_b.x_coord, point_b.y_coord) - Vec2::new(point_a.x_coord, point_a.y_coord)).angle_between(Vec2::new(point_a.x_coord, point_a.y_coord));

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(length_a_b, 1.0)),
                    ..default()
                },
                // transform: Transform::from_translation(Vec3::new(x as f32,y as f32, 1.0)).with_rotation(Quat::from_rotation_z(point_ab_angle)),
                transform: Transform::from_translation(Vec3::new(point_a.x_coord, point_a.y_coord, 1.0)).with_rotation(Quat::from_rotation_z(abs_difference_1)),
                ..default()
            });
            println!("length {:?}", length_a_b);
            // println!("{:?} {:?}", point_a, point_b );
            
        }
    }
       
    // for _ in 0..vec.capacity() {
    //     vec.push(rand::random());
    // };  
    
}

#[derive(Debug)]
struct pointVector {
    x_coord: f32,
    y_coord: f32 
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}