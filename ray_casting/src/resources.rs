use bevy::{asset::Handle, ecs::system::Resource, math::Vec2, render::mesh::Mesh, sprite::ColorMaterial, transform::*};

#[derive(Resource)]
pub struct Mouse {
    pub loc: Vec2,
}

impl Default for Mouse {
    fn default() -> Mouse {
        Mouse { loc: Vec2::new(0.0, 0.0) }
    }
}

#[derive(Resource, Default)]
pub struct RayAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

// impl Default for RayAssets {
//     fn default() -> RayAssets {
//         RayAssets { }
//     }
// }