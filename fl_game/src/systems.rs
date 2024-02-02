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
            rotation_speed: f32::to_radians(360.0)
        }
    ));
}

pub fn spawn_tourch(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&mut Transform, &Player, Entity),With<Player>>,
) {
    if let Ok((transform, player, player_entity)) = player_query.get_single_mut() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let tourch_points = vec![[0.0, 0.0, 0.0], [7.0, 4.0, 0.0], [7.0, -4.0, 0.0]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, tourch_points);
        let indices: Vec<u32> = vec![0, 2, 1];
        mesh.set_indices(Some(Indices::U32(indices)));
        let tourch = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5))),
                ..default()
            },
            Tourch{},
        )).id();
        commands.entity(player_entity).push_children(&[tourch]);
    }
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player),With<Player>>,
    time: Res<Time>
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
        }

        transform.translation += direction * player.movement_speed * time.delta_seconds();
    }
}

pub fn spawn_walls(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {

}