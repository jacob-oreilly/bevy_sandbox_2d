use bevy::{
    prelude::*, render::color, sprite::MaterialMesh2dBundle, transform::commands,
    window::PrimaryWindow,
};
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
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let rez: i64 = 20;
    let cols: i64 = (600 / rez);
    let rows = (400 / rez);
    let total_col_row = cols * rows;
    let mut column_major = Vec::new();

    for _n in 1..total_col_row {
        let mut rng = rand::thread_rng();
        let rand_point: i64 = rng.gen_range(0..=1);
        column_major.push(rand_point);
    }
    println!("{:?}", column_major);
    for y in 1..rows {
        let y_coord = (y * rez) as f32;
        for x in 1..cols {
            let x_coord = (x * rez) as f32;
            let current_index: usize = (y * (x - 1)).try_into().unwrap();
            let color_value = column_major[current_index] as f32;
            // Circle
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
                material: materials.add(
                    Color::rgba_linear(
                        color_value * 255.0,
                        color_value * 255.0,
                        color_value * 255.0,
                        color_value + 1.0,
                    )
                    .into(),
                ),
                transform: Transform::from_xyz(x_coord, y_coord, 0.0),
                ..default()
            });

            // println!("cols {:?} rows {:?}", cols, rows );
            // println!("point x {:?} point y {:?}", x_coord, y_coord );
            // println!("computed x {:?} coomputed y {:?}", x_coord - cols as f32, y_coord - rows as f32 );
        }
        println!()
    }

    for i in 1..rows {
        for j in 1..cols {
            let y = i * rez;
            let x = j * rez;
            let corner_a: usize = (i * (j - 1)).try_into().unwrap();
            let corner_b: usize = (i * j).try_into().unwrap();
            let corner_c: usize = ((i + 1) * j).try_into().unwrap();
            let corner_d: usize = ((i + 1) * (j - 1)).try_into().unwrap();

            let point_a = pointVector {
                x_coord: x as f32 + (rez as f32 * 0.5),
                y_coord: y as f32,
            };
            let point_b = pointVector {
                x_coord: (x + rez) as f32,
                y_coord: y as f32 + (rez as f32 * 0.5),
            };
            let point_c = pointVector {
                x_coord: (x + rez) as f32 * 0.5,
                y_coord: (y + rez) as f32,
            };
            let point_d = pointVector {
                x_coord: x as f32,
                y_coord: (y + rez) as f32 * 0.5,
            };

            let line_state = get_line_state(
                column_major[corner_a],
                column_major[corner_b],
                column_major[corner_c],
                column_major[corner_d],
            );

            match line_state {
                1 | 14 => {
                    println!("d-c");
                    spawn_line(point_d, point_c, &mut commands, line_state);
                },
                2 | 13 => {
                    println!("c-b");
                    spawn_line(point_c, point_b, &mut commands, line_state);
                },
                3 | 12 => {
                    println!("d-b");
                    spawn_line(point_d, point_b, &mut commands, line_state);
                },
                4 | 11 => {
                    println!("a-b");
                    spawn_line(point_a, point_b, &mut commands, line_state);
                },
                5 => {
                    println!("d-a & c-b");
                    spawn_line(point_d, point_a, &mut commands, line_state);
                    spawn_line(point_c, point_b, &mut commands, line_state);
                },
                6 | 9 => {
                    println!("a-c");
                    spawn_line(point_a, point_c, &mut commands, line_state);
                },
                7 | 8 => {
                    println!("d-a");
                    spawn_line(point_d, point_a, &mut commands, line_state);
                },
                10 => {
                    println!("a-b & d-c");
                    spawn_line(point_a, point_b, &mut commands, line_state);
                    spawn_line(point_d, point_c, &mut commands, line_state);
                },
                _ => (),
            }

            // spawn_line(point_a, point_b, &mut commands);

            // println!("a: {:?} b: {:?} c: {:?} d: {:?}", corner_a, corner_b, corner_c, corner_d);
            println!("Line State: {:?}", line_state);
        }
    }
}

#[derive(Debug)]
struct pointVector {
    x_coord: f32,
    y_coord: f32,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_line(point_start: pointVector, point_end: pointVector, commands: &mut Commands, line_state: i64) {
    let mut abs_difference = 0.0;
    let length;


    let point_end_vector = Vec2::new(point_end.x_coord, point_end.y_coord);
    let point_start_vector =  Vec2::new(point_start.x_coord, point_start.y_coord);
    let diff_vector = point_end_vector - point_start_vector;
    let abs_diff_str_1 = "1, 4, 10, 11, 14";
    let abs_diff_str_2 = "2, 5, 7, 8, 13, 15";
    if abs_diff_str_1.contains(&line_state.to_string()) {
        abs_difference = diff_vector.y.atan2(diff_vector.x) - (-std::f32::consts::FRAC_PI_4).abs()
        .to_degrees();
    }
    else if abs_diff_str_2.contains(&line_state.to_string()) {
        abs_difference = (diff_vector.y.atan2(diff_vector.x) - (3.0 * std::f32::consts::FRAC_PI_4)).abs()
        .to_degrees();
    }
    else {
        abs_difference = diff_vector.y.atan2(diff_vector.x) - (-std::f32::consts::FRAC_PI_4).abs()
        .to_degrees();
    }
        

    length = point_end_vector.distance(point_start_vector).abs();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(length, 1.0)),
            ..default()
        },
        // transform: Transform::from_translation(Vec3::new(point_start.x_coord, point_start.y_coord, 1.0)).with_rotation(Quat::from_rotation_z(abs_difference)),
            transform: Transform::from_translation(Vec3::new(point_start.x_coord, point_start.y_coord, 1.0)),
        ..default()
    });
}

fn get_line_state(a: i64, b: i64, c: i64, d: i64) -> i64 {
    // println!("a: {:?} b: {:?} c: {:?} d: {:?}", a as f32, b as f32, c as f32, d as f32);
    a * 8 + b * 4 + c * 2 + d * 1
}
