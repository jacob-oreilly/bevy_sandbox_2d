use std::{
    f32::{consts::{FRAC_PI_2, PI}, INFINITY},
    vec,
};

use bevy::{
    math::vec3,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
    transform,
    window::PrimaryWindow,
};

use crate::components::{self, Tourch, TourchLight, Wall};
use components::Player;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(25.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            },
            Player {
                movement_speed: 500.0,
                rotation_speed: f32::to_radians(360.0),
                player_rotation: f32::to_radians(0.0),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(25.0, 10.0)).into())
                        .into(),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_xyz(25.0, 0.0, 1.0),
                    ..default()
                },
                Tourch {},
            ));
            let rez = 180 / 90;
            for i in (0..1) {
                let angle = 0.0;
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(meshes.add(generate_line_mesh(
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(window.width(), 0.0, 0.0),
                        ))),
                        material: materials
                            .add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
                        ..default()
                    },
                    TourchLight {
                        point_a: Vec3::new(25.0, 0.0, 0.0),
                        point_b: Vec3::new(window.width(), angle, 0.0),
                        ray_direction: Vec2::from_angle(angle.to_radians()),
                    },
                ));
            }
        });
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            let current_direction = transform.translation.xy();
            transform.translation += direction * player.movement_speed * time.delta_seconds();
            let to_forward = (transform.translation.xy() - current_direction).normalize();
            let rotate_forward = Quat::from_rotation_arc(Vec3::X, to_forward.extend(0.0));
            transform.rotation = rotate_forward;
        }

        // println!("player: {:?}", transform.translation);
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let line_length = window.height();
    let line_start = Vec3::new(0.0, 0.0, 0.0);
    let line_end = Vec3::new(0.0, line_length, 0.0);
    let wall_mesh = generate_line_mesh(line_start, line_end);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(wall_mesh.clone()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 4.0, 0.0).with_scale(vec3(0.0, 0.5, 0.0)),
            ..default()
        },
        Wall {
            start: line_start,
            end: line_end,
        },
    ));

    //right wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(wall_mesh.clone()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            transform: Transform::from_xyz(window.width() - 1.0, 0.0, 0.0),
            ..default()
        },
        Wall {
            start: line_start,
            end: line_end,
        },
    ));
    
    //left wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(wall_mesh.clone()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },
        Wall {
            start: line_start,
            end: line_end,
        },
    ));

    let scale = (window.width() / line_length).abs();
    println!("Scale: {:?}",scale);
    //ceiling
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(wall_mesh.clone()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 1.0))),
            transform: Transform::from_xyz(window.width(), window.height() - 1.0, 0.0).with_scale(vec3(0.0, scale, 0.0)).with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),
            ..default()
        },
        Wall {
            start: line_start,
            end: line_end,
        },
    ));

    //floor
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(wall_mesh.clone()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 1.0))),
            transform: Transform::from_xyz(window.width(), 1.0, 0.0).with_scale(vec3(0.0, scale, 0.0)).with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),
            ..default()
        },
        Wall {
            start: line_start,
            end: line_end,
        },
    ));
}

pub fn tourch_light_update(
    mut commands: Commands,
    mut tourch_light_query: Query<
        (&mut TourchLight, Entity, &Parent, &mut Transform),
        With<TourchLight>,
    >,
    mut wall_query: Query<&mut Wall, Without<TourchLight>>,
    mut transform_query: Query<&Transform, Without<TourchLight>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let line_start = Vec3::new(0.0, 0.0, 0.0);
    let line_end = Vec3::new(1.0, 0.0, 0.0);
    let mut mesh = generate_line_mesh(line_start, line_end); 
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));

    for (tourch_light, tourch_light_entity, player, mut tourch_transform) in
        tourch_light_query.iter_mut()
    {
        // let tourch_global_position = tourch_global_transform.translation();
        let mut tourch_global_position = Vec3::ZERO;
        if let Ok(transform) = transform_query.get_mut(**player) {
            tourch_global_position = transform.translation;
            println!("Inside Player: {:?}", transform.translation);
        }

        // println!("Tourch global: {:?}", tourch_global_position);
        let mut closest_point: Vec3 = Vec3::NAN;
        let mut smallest_dist = f32::INFINITY;
        // println!("tourch_light_direct: {:?}",  tourch_light.ray_direction);
        // println!("tourch_light_org: {:?}",  tourch_global_position);
        for wall in wall_query.iter_mut() {
            let wall_vec1 = wall.start;
            let wall_vec2 = wall.end;
            let intersect_point = calc_intersect(
                wall_vec1,
                wall_vec2,
                tourch_global_position,
                tourch_light.ray_direction,
            );
            
            if intersect_point != None {
                println!("Intersect Point: {:?}", intersect_point );
                let temp_cursor_vector = tourch_global_position;
                let distance = temp_cursor_vector.distance(intersect_point.unwrap());
                if distance < smallest_dist {
                    smallest_dist = distance;
                    closest_point = intersect_point.unwrap();
                }
            }
        }
        if closest_point != Vec3::NAN {
            let tourch_position = vec![
                [0.0, 0.0, 0.0],
                [closest_point.x, closest_point.y, 0.0],
            ];
            println!("tourch position: {:?}", tourch_position );
            // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, tourch_position);
            // let scale = 
            // tourch_transform.translation = Transform::from_scale(scale);
            let scale_x = (closest_point.x / tourch_global_position.x).abs();  
            let scale_y = (closest_point.y / tourch_global_position.y).abs();
            println!("Tourch scale x: {:?}", scale_x);
            println!("Tourch scale y: {:?}", scale_y);
            tourch_transform.scale = Vec3::new(scale_x, scale_y, 0.0);
            println!("Tourch scale: {:?}", tourch_transform.scale);
            // commands
            //     .entity(tourch_light_entity)
            //     .insert(MaterialMesh2dBundle {
            //         mesh: meshes.add(mesh.clone()).into(),
            //         material: materials
            //             .add(ColorMaterial::from(Color::rgba_linear(1.0, 1.0, 1.0, 0.5))),
            //         ..default()
            //     });
        }
    }
}

pub fn calc_intersect(
    point_a: Vec3,
    point_b: Vec3,
    tourch: Vec3,
    ray_direction: Vec2,
) -> Option<Vec3> {
    let mut intersect_vec = Vec3::ZERO;
    // println!("point_a : {:?}, point_b: {:?}", point_a, point_b);
    let x1 = point_a.x;
    let y1 = point_a.y;
    let x2 = point_b.x;
    let y2 = point_b.y;

    let x3 = tourch.x;
    let y3 = tourch.y;
    let x4 = tourch.x + ray_direction.x;
    let y4 = tourch.y + ray_direction.y;

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

    if t > 0.0 && t < 1.0 && u > 0.0 {
        intersect_vec.x = x1 + t * (x2 - x1);
        intersect_vec.y = y1 + t * (y2 - y1);
        return Some(intersect_vec);
    } else {
        None
    }
}

pub fn generate_line_mesh(line_start: Vec3, line_end: Vec3) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let line_vector = vec![line_start, line_end];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line_vector);
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

pub fn update_tourch_light_direction(
    keys: Res<Input<KeyCode>>,
    mut tourch_light_query: Query<(&mut Transform, &mut TourchLight), With<TourchLight>>,
) {
    for (mut transform, mut tourch_light) in tourch_light_query.iter_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction += Vec2::new(1.0, 0.0);
        }
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            direction += Vec2::new(0.0, 1.0);
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            direction += Vec2::new(0.0, -1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            tourch_light.ray_direction += direction;
        }
 
        // if direction.length() > 0.0 {
        //     direction = direction.normalize();
        //     let current_direction = transform.translation.xy();
        //     transform.translation += direction * player.movement_speed * time.delta_seconds();
        //     let to_forward = (transform.translation.xy() - current_direction).normalize();
        //     let rotate_forward = Quat::from_rotation_arc(Vec3::X, to_forward.extend(0.0));
        //     transform.rotation = rotate_forward;
        // }

        // println!("player: {:?}", transform.translation);
    }
}
