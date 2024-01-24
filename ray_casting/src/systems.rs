use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::{components, resources::{Mouse, RayAssets}, MainCamera, RayDirection, Wall};
use components::Ray;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_cursor: ResMut<Mouse>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    my_cursor.loc = Vec2::new(0.0, 0.0);

    let box_v1 = Vec3::new(0.0, 150.0, 0.0);
    let box_v2 = Vec3::new(2.0, -150.0, 0.0);
    let translation_vec = Vec3::new(0.0, 20.0, 0.);
    //Wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(shape::Box::from_corners(box_v1, box_v2).into())),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(translation_vec),
            ..default()
        },
        Wall {
            point_a: box_v1,
            point_b: box_v2,
        },
    ));
}

pub fn draw_rays(
    mut commands: Commands,
    my_cursor: ResMut<Mouse>,
    ray_assets: ResMut<RayAssets>
) {
    let x_offset = 20. / 2.0 + my_cursor.loc.x;
    let ray_coord = Vec3::new(x_offset, my_cursor.loc.y, 0.0); 
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(ray_assets.mesh.clone()),
            material: ray_assets.material.clone(),
            transform: Transform::from_translation(ray_coord),
            ..default()
        },
        Ray {},
        RayDirection{
            direction_x: 1.0,
            direction_y: 0.0
        }
    ));
}

pub fn ray_intersect_update(
    mut commands: Commands,
    mut ray_query: Query<(&mut Transform, Entity, &mut RayDirection), With<Ray>>,
    mut wall_query: Query<&mut Wall, Without<Ray>>,
    mut ray_assets: ResMut<RayAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    my_cursor: ResMut<Mouse>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let (ray_transform, ray_entity, ray_direction) = ray_query.get_single_mut().unwrap();
    let window = window_query.get_single().unwrap();

    for wall in wall_query.iter_mut() {
        let wall_vec1 = wall.point_a;
        let wall_vec2 = wall.point_b;
        let intersect_point = calc_intersect(wall_vec1, wall_vec2, &my_cursor, &ray_direction);
        let x_offset: f32;
        let ray_coord: Vec3;
        let current_ray_assets: RayAssets;
        
        if intersect_point != None {
            let intersect_distance = intersect_point.unwrap() - ray_transform.translation;
            x_offset = intersect_distance.x / 2.0 + my_cursor.loc.x;
            ray_coord = Vec3::new(x_offset, my_cursor.loc.y, 0.0);
            current_ray_assets = RayAssets {
                mesh: meshes.add(shape::Quad::new(Vec2::new(intersect_distance.x, 2.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
            };
            commands.entity(ray_entity).insert(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(current_ray_assets.mesh),
                material: current_ray_assets.material,
                transform: Transform::from_translation(ray_coord),
                ..default()
            });
            ray_assets.mesh = meshes.add(shape::Quad::new(Vec2::new(intersect_distance.x, 2.)).into()).into();
        }
        else {
            let x = window.width() - ray_transform.translation.x;
            x_offset = x / 2.0 + my_cursor.loc.x;
            ray_coord = Vec3::new(x_offset, my_cursor.loc.y, 0.0);
            current_ray_assets = RayAssets {
                mesh: meshes.add(shape::Quad::new(Vec2::new(x, 2.)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
            };
            commands.entity(ray_entity).insert(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(current_ray_assets.mesh),
                material: current_ray_assets.material,
                transform: Transform::from_translation(ray_coord),
                ..default()
            });
            ray_assets.mesh = meshes.add(shape::Quad::new(Vec2::new(x, 2.)).into()).into();
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

pub fn calc_intersect(point_a: Vec3, point_b: Vec3, my_cursor: &ResMut<Mouse>, ray_direction: &Mut<RayDirection>) -> Option<Vec3> {
    let mut intersect_vec = Vec3::ZERO;
    let x1 = point_a.x;
    let y1 = point_a.y;
    let x2 = point_b.x;
    let y2 = point_b.y; 

    let x3 = my_cursor.loc.x;
    let y3 = my_cursor.loc.y;
    let x4 = my_cursor.loc.x + ray_direction.direction_x;
    let y4 = my_cursor.loc.y + ray_direction.direction_y;

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

    if t > 0.0 && t < 1.0 && u > 0.0 {
        intersect_vec.x = x1 + t * (x2 - x1);
        intersect_vec.y = y1 + t * (y2 - y1);
        return Some(intersect_vec.normalize());
    }
    None
}

//This idea will be used for different facing vectors
// ray_direction.direction_x = wall_vec1.x - my_cursor.loc.x;
// ray_direction.direction_y = wall_vec1.y - my_cursor.loc.y;