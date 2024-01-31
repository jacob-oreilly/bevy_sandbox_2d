use std::vec;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};

use crate::{
    components,
    resources::{Mouse, NumberOfRays},
    MainCamera, Wall,
};
use components::Ray;

pub fn setup(mut commands: Commands, mut my_cursor: ResMut<Mouse>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    my_cursor.loc = Vec2::new(0.0, 0.0);
}

pub fn draw_walls(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let mut walls: Vec<Vec<[f32; 3]>> = Vec::new();
    let left_border = vec![
        [-(window.width() / 2.0), window.height() / 2.0, 0.0],
        [-(window.width() / 2.0), -window.height() / 2.0, 0.0],
    ];
    let right_border = vec![
        [window.width() / 2.0, window.height() / 2.0, 0.0],
        [window.width() / 2.0, -window.height() / 2.0, 0.0],
    ];
    let top_border = vec![
        [-(window.width() / 2.0), window.height() / 2.0, 0.0],
        [window.width(), window.height() / 2.0, 0.0],
    ];
    let bottom_border = vec![
        [-(window.width() / 2.0), -window.height() / 2.0, 0.0],
        [window.width(), -window.height() / 2.0, 0.0],
    ];
    let center_wall = vec![[-40.0, 200.0, 0.0], [40.0, -200.0, 0.0]];
    let normal_wall_1 = vec![[-90.0, 200.0, 0.0], [100.0, 200.0, 0.0]];
    let normal_wall_2 = vec![[-20.0, 200.0, 0.0], [60.0, -200.0, 0.0]];
    let normal_wall_3 = vec![[100.0, 200.0, 0.0], [40.0, -400.0, 0.0]];
    walls.push(left_border);
    walls.push(right_border);
    walls.push(top_border);
    walls.push(bottom_border);
    walls.push(center_wall);
    walls.push(normal_wall_1);
    walls.push(normal_wall_2);
    walls.push(normal_wall_3);

    for wall in walls {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, wall.clone());
        let indices: Vec<u32> = vec![0, 1];
        mesh.set_indices(Some(Indices::U32(indices)));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                ..default()
            },
            Wall {
                point_a: wall[0].into(),
                point_b: wall[1].into(),
            },
        ));
    }
}

pub fn draw_rays(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    number_of_rays: ResMut<NumberOfRays>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let ray_start = vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, ray_start);
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));

    let ray_increment = 360 / number_of_rays.num;
    println!("Ray_increment: {:?}", ray_increment);

    for angle in (0..360).step_by(ray_increment.try_into().unwrap()) {
        println!("Ray angle: {:?}", angle);
        println!(
            "Ray direction: {:?}",
            Vec2::from_angle((angle as f32).to_radians())
        );
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh.clone()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            },
            Ray {
                point_a: Vec3::new(0.0, 0.0, 0.0),
                point_b: Vec3::new(0.0, 0.0, 0.0),
                ray_direction: Vec2::from_angle((angle as f32).to_radians()),
            },
        ));
    }
}

pub fn ray_intersect_update(
    mut commands: Commands,
    mut ray_query: Query<(&mut Ray, Entity), With<Ray>>,
    mut wall_query: Query<&mut Wall, Without<Ray>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    my_cursor: ResMut<Mouse>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));

    for (ray, ray_entity) in ray_query.iter_mut() {
        let ray_direction = ray.ray_direction;
        // println!("Ray direction: {:?}", ray_direction);
        let mut closest_point: Vec3 = Vec3::NAN;
        let mut smallest_dist = f32::INFINITY;
        for wall in wall_query.iter_mut() {
            let wall_vec1 = wall.point_a;
            let wall_vec2 = wall.point_b;
            let intersect_point = calc_intersect(wall_vec1, wall_vec2, &my_cursor, ray_direction);
            if intersect_point != None {
                let temp_cursor_vector = Vec3::new(my_cursor.loc.x, my_cursor.loc.y, 0.0);
                let distance = temp_cursor_vector.distance(intersect_point.unwrap());
                if distance < smallest_dist {
                    smallest_dist = distance;
                    closest_point = intersect_point.unwrap();
                }
            }
        }
        if closest_point != Vec3::NAN {
            let ray_start = vec![
                [my_cursor.loc.x, my_cursor.loc.y, 0.0],
                [closest_point.x, closest_point.y, 0.0],
            ];
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, ray_start);
            commands.entity(ray_entity).insert(MaterialMesh2dBundle {
                mesh: meshes.add(mesh.clone()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            });
        }
        
    }
}

pub fn cursor_events(
    mut mouse_coords: ResMut<Mouse>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.loc = world_position;
    }
}

pub fn calc_intersect(
    point_a: Vec3,
    point_b: Vec3,
    my_cursor: &ResMut<Mouse>,
    ray_direction: Vec2,
) -> Option<Vec3> {
    let mut intersect_vec = Vec3::ZERO;
    // println!("point_a : {:?}, point_b: {:?}", point_a, point_b);
    let x1 = point_a.x;
    let y1 = point_a.y;
    let x2 = point_b.x;
    let y2 = point_b.y;

    let x3 = my_cursor.loc.x;
    let y3 = my_cursor.loc.y;
    let x4 = my_cursor.loc.x + ray_direction.x;
    let y4 = my_cursor.loc.y + ray_direction.y;

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
