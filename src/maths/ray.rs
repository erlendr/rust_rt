use maths::vector3d::Vector3D;

pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3D { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}
