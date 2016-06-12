use maths::vector3d::Vector3D;
use maths::color::Color;

pub struct Light {
    pub intensity: f64,
    pub position: Vector3D,
    pub color: Color
}

impl Light {
    pub fn new() -> Light {
        return Light {
            intensity: 1.0,
            position: Vector3D { x: 0.0 , y: 0.0, z: 0.0 },
            color: Color::new()
        }
    }
}
