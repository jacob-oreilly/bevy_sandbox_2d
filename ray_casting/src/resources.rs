use bevy::{asset::Handle, ecs::system::Resource, math::Vec2, render::mesh::Mesh, sprite::ColorMaterial};

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

#[derive(Resource)]
pub struct NumberOfRays {
    pub num: u32
}

impl Default for NumberOfRays {
    fn default() -> NumberOfRays {
        NumberOfRays {
            num: 12
        }
    }
}

#[derive(Resource)]
pub struct RayRotation {
    pub rotation: f32
}

impl Default for RayRotation {
    fn default() -> RayRotation {
        RayRotation {
            rotation: 16.0
        }
    }
}
