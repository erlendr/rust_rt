pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        return ((self.x*self.x) + (self.y*self.y) + (self.z*self.z) as f64).sqrt();
    }

    pub fn normalize(&self) -> Vector3D {
        let magnitude = self.magnitude();
        return Vector3D {x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude };
    }

    pub fn negative(&self) -> Vector3D {
        return Vector3D {x: -self.x, y: -self.y, z: -self.z };
    }

    pub fn dot_product(&self, v2: &Vector3D) -> f64 {
        return (self.x * v2.x) + (self.y * v2.y) + (self.z * v2.z);
    }

    pub fn cross_product(&self, v2: &Vector3D) -> Vector3D {
        return Vector3D {
            x: ((self.y * v2.z) - (self.z * v2.y)),
            y: ((self.z * v2.x) - (self.x * v2.z)),
            z: ((self.x * v2.y) - (self.y * v2.x))
        };
    }

    pub fn add(&self, v2: &Vector3D) -> Vector3D {
        return Vector3D {
            x: (self.x + v2.x),
            y: (self.y + v2.y),
            z: (self.z + v2.z)
        };
    }

    pub fn mult(&self, scalar: f64) -> Vector3D {
        return Vector3D {
            x: (self.x * scalar),
            y: (self.y * scalar),
            z: (self.z * scalar)
        };
    }
}
