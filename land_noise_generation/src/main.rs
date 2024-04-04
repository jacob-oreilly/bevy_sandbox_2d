use bevy::{
    prelude::*, render::{color, render_resource::encase::rts_array::Length}, sprite::MaterialMesh2dBundle, transform::commands,
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
        .add_systems(Update, cursor_events)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let rez: i64 = 50;
    let cols: i64 = (600 / rez) + 1;
    let rows = (400 / rez) + 1;
    // let rows = 4;
    // let cols = 16;
    let total_col_row = cols * rows;
    let mut column_major = Vec::new();
    // let column_major = vec![0,0,1,1,1,0,0,1,0,1,1,0,0,0,0,0,1,1,0,0,1,0,0,1,0,0,0,0,1,0,0,1,1,0,0,1,1,1,0,0,1,0,0,1,1,1,1,1,0,1,1,0,1,1,0,0,1,1,1,1,0,1,1,0];

    for _n in 0..total_col_row {
        let mut rng = rand::thread_rng();
        let rand_point: i64 = rng.gen_range(0..=1);
        column_major.push(rand_point);
    }
    println!("{:?}", column_major.length());
    println!("{:?}", column_major);
    for y in 0..rows {
        let y_coord = (y * rez) as f32;
        // let mut current_index = 0;
        for x in 0..cols {
            let x_coord = (x * rez) as f32;
            let current_index: usize = (((y * cols) + x)).try_into().unwrap();
            println!("grid current index: {:?}", current_index);
            let color_value = column_major[current_index] as f32;
            // Circle
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(2.0)).into(),
                material: materials.add(
                    Color::rgba_linear(
                        color_value * 255.0,
                        color_value * 255.0,
                        color_value * 255.0,
                        color_value + 1.0,
                    ),
                ),
                transform: Transform::from_xyz(x_coord, y_coord, 0.0),
                ..default()
            });
            println!("point x {:?} point y {:?}", x_coord, y_coord );
        }
    }

    for i in 0..rows {
        let y = i * rez;
        for j in 0..cols {
            let x = j * rez;
            let current_y = i * (cols);
            if (current_y + (j + 1) + cols) >= column_major.len() as i64 {
                break;
            }
            let corner_a: usize = (current_y + j).try_into().unwrap();
            let corner_b: usize = (current_y + (j + 1)).try_into().unwrap();
            let corner_d: usize = (current_y + j + cols).try_into().unwrap();
            let corner_c: usize = (current_y + (j + 1) + cols).try_into().unwrap();
            println!("{:?}", current_y);


            let point_a = pointVector {
                x_coord: x as f32 + (rez as f32 * 0.5),
                y_coord: y as f32,
            };
            let point_b = pointVector {
                x_coord: (x + rez) as f32,
                y_coord: y as f32 + (rez as f32 * 0.5),
            };
            let point_c = pointVector {
                x_coord: x as f32 + (rez as f32 * 0.5),
                y_coord: (y + rez) as f32,
            };
            let point_d = pointVector {
                x_coord: x as f32,
                y_coord: y as f32 + (rez as f32 * 0.5),
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
                    spawn_line(point_d, point_c, &mut commands, line_state, rez);
                }
                2 | 13 => {
                    println!("c-b");
                    spawn_line(point_c, point_b, &mut commands, line_state, rez);
                }
                3 | 12 => {
                    println!("d-b");
                    spawn_line(point_d, point_b, &mut commands, line_state, rez);
                }
                4 | 11 => {
                    println!("a-b");
                    spawn_line(point_a, point_b, &mut commands, line_state, rez);
                }
                5 => {
                    println!("d-a & c-b");
                    spawn_line(point_d, point_a, &mut commands, line_state, rez);
                    spawn_line(point_c, point_b, &mut commands, line_state, rez);
                }
                6 | 9 => {
                    println!("a-c");
                    spawn_line(point_a, point_c, &mut commands, line_state, rez);
                }
                7 | 8 => {
                    println!("d-a");
                    spawn_line(point_d, point_a, &mut commands, line_state, rez);
                }
                10 => {
                    println!("a-b & d-a");
                    spawn_line(point_a, point_b, &mut commands, line_state, rez);
                    spawn_line(point_d, point_c, &mut commands, line_state, rez);
                }
                _ => (),
            }
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
        transform: Transform::from_xyz(window.width() / 2.0 - 25.0, window.height() / 2.0 -25.0, 0.0),
        ..default()
    });
}

fn spawn_line(
    point_start: pointVector,
    point_end: pointVector,
    commands: &mut Commands,
    line_state: i64,
    rez: i64,
) {
    let mut abs_difference;
    let mut length = 0.0;
    let color: Color;

    let point_end_vector = Vec2::new(point_end.x_coord, point_end.y_coord);
    let point_start_vector = Vec2::new(point_start.x_coord, point_start.y_coord);
    let diff_vector = point_end_vector - point_start_vector;
    let abs_diff_str_1 = [1, 4, 10, 11, 14];
    let abs_diff_str_2 = [2, 5, 7, 8, 13, 15];
    let abs_diff_str_3 = [6, 9];
    let mut x_start_coord_offset = point_start.x_coord;
    let mut y_start_coord_offset = point_start.y_coord;

    if abs_diff_str_1.contains(&line_state) {
        abs_difference = diff_vector.y.atan2(diff_vector.x);
        color = Color::BLUE;
        length = point_end_vector.distance(point_start_vector);
        x_start_coord_offset = x_start_coord_offset + (rez as f32 * 0.25);
        y_start_coord_offset = y_start_coord_offset + (rez as f32 * 0.25);
    }
    else if abs_diff_str_2.contains(&line_state) {
        abs_difference = diff_vector.y.atan2(diff_vector.x);
        color = Color::RED;
        length = point_end_vector.distance(point_start_vector);
        x_start_coord_offset = x_start_coord_offset + (rez as f32 * 0.25);
        y_start_coord_offset = y_start_coord_offset - (rez as f32 * 0.25);
    }
    else if abs_diff_str_3.contains(&line_state){
        abs_difference = std::f32::consts::FRAC_PI_2;
        color = Color::GREEN;
        length = point_end_vector.distance(point_start_vector);
        y_start_coord_offset = y_start_coord_offset + (rez as f32 * 0.5);
    }
    else {
        abs_difference = 0.0;
        color = Color::GREEN;
        length = point_end_vector.distance(point_start_vector);
        x_start_coord_offset = x_start_coord_offset + (rez as f32 * 0.5);
        
    }
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: color,
            custom_size: Some(Vec2::new(length, 1.0)),
            ..default()
        },
        transform: Transform::from_translation(
            Vec3::new(
            x_start_coord_offset,
            y_start_coord_offset,
            1.0,
            ))
        .with_rotation(Quat::from_rotation_z(abs_difference)),
        ..default()
    });
    println!(
        "line_state: {:?} point_start: {:?} point_end: {:?} length: {:?}",
        line_state, point_start, point_end, length
    );

    
}

fn get_line_state(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * 8 + b * 4 + c * 2 + d * 1
}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    for ev in cursor_evr.read() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x / 2.0, ev.position.y / 2.0, ev.window
        );
    }
}
