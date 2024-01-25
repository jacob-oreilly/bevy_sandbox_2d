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
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0], [0.0, 400.0, 0.0]]);
    let indices: Vec<u32> = vec![0, 1];
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(200.0, -200.0,0.0)),
        ..default()
    });
}
