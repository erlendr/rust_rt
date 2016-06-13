use maths::vector3d::Vector3D;

pub struct Camera {
    pub position: Vector3D,
    pub direction: Vector3D,
    pub right: Vector3D,
    pub down: Vector3D,
}

impl Camera {
    pub fn new() -> Camera {
        return Camera {
            position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
            right: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            down: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}
