use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::color};
use rand::Rng;


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
    let rez: i64 = 15;
    let cols: i64 = 1400 / rez;
    let rows = 800 / rez;
    let total_col_row = cols * rows;
    let mut column_major = Vec::new();

    commands.spawn(Camera2dBundle::default());

    for _n in 1..total_col_row {
        let mut rng = rand::thread_rng();
        let rand_point: i64 = rng.gen();
        column_major.push(rand_point);
    }
    println!("{:?}", column_major);
    for y in 1..rows {
        let y_coord = (y * rez) as f32;
        for x in 1..cols {
            let x_coord = (x * rez) as f32;
            let current_index: usize = (y * (x - 1)).try_into().unwrap();
            let color_value = column_major[current_index] as f32;
            println!("{:?}", color_value);
            // Circle
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
                material: materials.add(Color::rgba_linear(color_value * 255.0, color_value * 255.0, color_value * 255.0, color_value).into()),
                transform: Transform::from_translation(Vec3::new(-x_coord, y_coord - 50.0, 0.)),
                ..default()
            });
        }
        }
       
    // for _ in 0..vec.capacity() {
    //     vec.push(rand::random());
    // };  
    
}
