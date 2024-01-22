use std::sync::Arc;

use bevy::{
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2d},
    transform,
    window::PrimaryWindow,
};

#[derive(Component)]
struct Ray {}

#[derive(Component)]
struct Wall {
    point_a: Vec2,
    point_b: Vec2,
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
    //Wall
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(2.0, 300.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(100.0, 20.0, 0.)),
            ..default()
        },
        Wall {
            point_a: Vec2::new(100.0, 300.0 / 2.0),
            point_b: Vec2::new(100.0, -300.0 / 2.0),
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
    mut ray_query: Query<(&mut Transform, Entity, &mut Handle<ColorMaterial>), With<Ray>>,
    mut wall_query: Query<&Transform, Without<Ray>>,
    mut ray_assets: ResMut<RayAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_cursor: ResMut<Mouse>,
    ) {
    //Where we will do the calculations for the intersections
    let (ray_transform, ray_entity, ray_handle) = ray_query.get_single_mut().unwrap();
    let x_offset = 20. / 2.0 + my_cursor.loc.x;
    let ray_coord = Vec3::new(x_offset, my_cursor.loc.y, 0.0); 
    for wall in wall_query.iter_mut() {
        let distance = ray_transform.translation.distance(wall.translation);
        let current_ray_assets = RayAssets {
            mesh: meshes.add(shape::Quad::new(Vec2::new(distance, 2.)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
        };
        commands.entity(ray_entity).insert(MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(current_ray_assets.mesh),
            material: current_ray_assets.material,
            transform: Transform::from_translation(ray_coord),
            ..default()
        });
        // println!("Ray Entity: {:?}",  );
        if distance.abs() > 0.0 {
            ray_assets.mesh = meshes.add(shape::Quad::new(Vec2::new(distance, 2.)).into()).into();
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
