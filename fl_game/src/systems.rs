use std::process::CommandArgs;

use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::components::{self, Tourch};
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
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player),With<Player>>,
    time: Res<Time>
) {

    if let Ok((mut transform,  mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        let mut rotation: f32 = -1.0;
        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            rotation = 180.0;
        }
        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            rotation = 0.0;
        }
        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            rotation = 90.0;
        }
        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            rotation = -90.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.movement_speed * time.delta_seconds();
        }
        if rotation > -1.0 && player.player_rotation != rotation {
            player.player_rotation = rotation;
            transform.rotate_z(player.player_rotation.to_radians());
        }
        
        
    }
}

pub fn spawn_walls(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {

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