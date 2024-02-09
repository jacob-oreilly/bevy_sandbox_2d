use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::components::{self, Tourch, Wall};
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
    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
            player_rotation: f32::to_radians(0.0),
        }
    )).with_children(|parent| {
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(10.0, 5.0)).into()).into(),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(10.0, 0.0, 1.0),
                ..default()
            },
            Tourch{},
        ));
    });
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player),With<Player>>,
    time: Res<Time>
) {

    if let Ok((mut transform,  player)) = player_query.get_single_mut() {
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
        
        
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let line_start = Vec3::new(window.width() - 30.0, window.height() / 2.0, 0.0);
    let line_end = Vec3::new(window.width() - 30.0, 0.0, 0.0 );
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(generate_line_mesh(line_start, line_end))),
            material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
            ..default()
        },
        Wall {}    
    ));
}


// pub fn spawn_tourch(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut player_query: Query<(&mut Transform, &Player, Entity),With<Player>>,
//     window_query: Query<&Window, With<PrimaryWindow>>
// ) {

//     let window = window_query.get_single().unwrap();
//     let center_x = window.width() / 2.0;
//     let center_y = window.height() / 2.0;
//     if let Ok((transform, player, player_entity)) = player_query.get_single_mut() {
//         let tourch = commands.spawn((
//             PbrBundle {
//                 mesh: meshes.add(shape::Quad::new(Vec2::new(10.0, 5.0)).into()).into(),
//                 material: materials.add(Color::RED.into()),
//                 ..default()
//             },
//             Tourch{},
//         )).id();
//         commands.entity(player_entity).push_children(&[tourch]);
//     }
// }

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