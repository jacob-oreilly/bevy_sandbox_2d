use std::sync::Arc;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2d},
    transform,
    window::PrimaryWindow,
};

#[derive(Component)]
struct Ray {}

#[derive(Component)]
struct Wall {
    point_a: Vec3,
    point_b: Vec3,
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct Mouse {
    loc: Vec2,
}

#[derive(Resource, Default)]
struct RayAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct RayDirection {
    direction_x: f32,
    direction_y: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_rays))
        .add_systems(Update, (cursor_events, ray_intersect_update, ray_position_update))
        .insert_resource(Mouse::default())
        .insert_resource(RayAssets::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut my_cursor: ResMut<Mouse>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((Camera2dBundle::default(), MainCamera));
    my_cursor.loc = Vec2::new(0.0, 0.0);

    let box_v1 = Vec3::new(0.0, 150.0, 0.0);
    let box_v2 = Vec3::new(2.0, -150.0, 0.0);
    //Wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(shape::Box::from_corners(box_v1, box_v2).into())),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(100.0, 20.0, 0.)),
            ..default()
        },
        Wall {
            point_a: box_v1,
            point_b: box_v2,
        },
    ));

    //Ray
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 2.)).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::WHITE)),
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //         ..default()
    //     },
    //     Ray{

    //     },
    // ));
    // println!("Ray Casting");
}

fn draw_rays(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut my_cursor: ResMut<Mouse>,
    mut ray_assets: ResMut<RayAssets>
) {
    let x_offset = 20. / 2.0 + my_cursor.loc.x;
    // ray_assets.mesh = meshes.add(shape::Quad::new(Vec2::new(20.0, 2.)).into()).into();
    // ray_assets.material = materials.add(ColorMaterial::from(Color::WHITE));
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
            direction_x: x_offset,
            direction_y: my_cursor.loc.y
        }
    ));
}

fn ray_position_update(
    cursor: ResMut<Mouse>,
    mut ray_query: Query<(&mut Transform), With<Ray>>,

) {
    let x_offset = 20. / 2.0 + cursor.loc.x;
    let ray_coord = Vec3::new(x_offset, cursor.loc.y, 0.0); 
    let mut ray_transform = ray_query.get_single_mut().unwrap();
    ray_transform.translation = ray_coord;
}
fn ray_intersect_update(
    mut commands: Commands,
    mut ray_query: Query<(&mut Transform, Entity, &mut Handle<ColorMaterial>, &mut RayDirection), With<Ray>>,
    mut wall_query: Query<(&Transform, &mut Wall), Without<Ray>>,
    mut ray_assets: ResMut<RayAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_cursor: ResMut<Mouse>,
    ) {
    //Where we will do the calculations for the intersections
    let (ray_transform, ray_entity, ray_handle, mut ray_direction) = ray_query.get_single_mut().unwrap();
    
    
    for (wall_transform, wall) in wall_query.iter_mut() {
        // let distance = ray_transform.translation.distance(wall.translation);
        let wall_vec1 = wall.point_a;
        let wall_vec2 = wall.point_b;
        //Here we will start the math
        let is_intersect = calc_intersect(wall_vec1, wall_vec2, &my_cursor, &ray_direction);
        if is_intersect {
            println!("Is intersect: {:?}", is_intersect);
        }
        
        let x1 = wall_transform.translation;
        // println!("wall_translation: {:?}", x1);
        let distance = wall_transform.translation - ray_transform.translation;
        let x_offset = distance.x / 2.0 + my_cursor.loc.x;
        let ray_coord = Vec3::new(x_offset, my_cursor.loc.y, 0.0); 
        let current_ray_assets = RayAssets {
            mesh: meshes.add(shape::Quad::new(Vec2::new(distance.x, 2.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
        };
        
        if distance.x > 0.0 {
            commands.entity(ray_entity).insert(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(current_ray_assets.mesh),
                material: current_ray_assets.material,
                transform: Transform::from_translation(ray_coord),
                ..default()
            });
            ray_assets.mesh = meshes.add(shape::Quad::new(Vec2::new(distance.x, 2.)).into()).into();
        }
    }   
    
}

fn cursor_events(
    mut mouse_coords: ResMut<Mouse>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    cursor_evr: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.loc = world_position;
        println!(
            "World coords y an x: {:?},{:?}",
            world_position.x, world_position.y
        );
    }
}

fn calc_intersect(point_a: Vec3, point_b: Vec3, my_cursor: &ResMut<Mouse>, ray_direction: &Mut<RayDirection>) -> bool {
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
        return false;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

    if t > 0.0 && t < 1.0 && u > 0.0{
        return true;
    }
    return false;
}   
// fn cast_update(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut ray_query: Query<(&mut Transform), With<Ray>>,
//     mut wall_query: Query<&Transform, Without<Ray>>,
//     time: Res<Time>,
// ) {
//     // let wall = wall_query.get_single().unwrap();
//     if let Ok((mut ray_transform)) = ray_query.get_single_mut() {
//         let mut ray_translation = ray_transform.translation;
//         let mut ray_ax = ray_translation.x;
//         let ray_ay = ray_translation.y;
//         let ray_bx = ray_translation.x + 1.0;
//         let ray_by = ray_translation.y;
//         for wall in wall_query.iter_mut() {
//             let wall_translation = wall.translation;
//             let wall_ax = wall_translation.x;
//             let wall_ay = wall_translation.y + 150.0;
//             let wall_bx = wall_translation.x;
//             let wall_by = wall_translation.y - 150.0;
//             let distance = ray_transform.translation.distance(wall.translation);
//             // println!("Ray Dist: {:?}", distance);
//             if distance.abs() > 0.0 {
//                 ray_translation = Vec3::new(distance, 2.0, 0.0);
//             }
//         }
//     }
// }
