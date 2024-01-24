use bevy::{prelude::*, render::{mesh::{Indices, MeshVertexAttribute}, render_resource::{PrimitiveTopology, VertexFormat}}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 0.1], [0.0, 0.0, 0.1], [0.0, 0.0, 0.1]]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0,1.0],[1.0,0.0],[1.0,1.0]]);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, values)
    // mesh.compute_flat_normals();
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::default().with_scale(Vec3::splat(128.0)),
        ..default()
    });
}
