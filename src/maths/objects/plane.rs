use maths::color::Color;
use maths::vector3d::Vector3D;
use maths::ray::Ray;
use maths::objects::object;

pub struct Plane {
    pub color: Color,
    pub normal: Vector3D, // n
    pub distance: f64,    // p0
}

impl Plane {
    pub fn new() -> Plane {
        return Plane {
            color: Color { red: 0.5, green: 0.5, blue: 0.5, special: 0.0 },
            normal: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            distance: 0.0,
        };
    }
}

impl object::Object for Plane {
    fn get_normal_at(&self, point: &Vector3D) -> Vector3D {
        let _ = point;

        return Vector3D {
            x: self.normal.x,
            y: self.normal.y,
            z: self.normal.z
        };
    }

    fn find_intersection(&self, ray: &Ray) -> f64 {
        //http://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection

        let a = ray.direction.dot_product(&self.normal); // l dot n
        if a == 0.0 {
            //Ray is parallel to the plane (perpendicular) because angle between them is zero
            return -1.0;
        }

        // (l0 - n*p0) dot n
        let b = self.normal.dot_product(
            &ray.origin.add(
                &self.normal.mult(self.distance).negative()
            )
        );

        return -1.0 * b / a; //Distance from ray origin to point of intersection (d)
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use maths::vector3d::Vector3D;
    use maths::ray::Ray;
    use maths::color::Color;
    use maths::objects::object::Object;

    #[test]
    fn get_normal_at_valid_input_should_return_correct_result() {
        let plane = Plane::new();
        let point = Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        let result = plane.get_normal_at(&point);

        assert_eq!(1.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(0.0, result.z);

    }

    #[test]
    fn find_intersection_rayisperpendiculartoplane_should_return_correct_result() {
        let plane = Plane {
            color: Color { red: 0.5, green: 0.5, blue: 0.5, special: 0.0 },
            normal: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
            distance: -1.0,
        };

        let ray = Ray {
            origin: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3D { x: 0.0, y: 1.0, z: 0.0 }
        };

        let result = plane.find_intersection(&ray);
        assert_eq!(-1.0, result);
    }

    #[test]
    fn find_intersection_rayiscrossingplane_should_return_correct_result() {
        let plane = Plane::new();
        let ray = Ray {
            origin: Vector3D { x: -2.0, y: -2.0, z: -2.0 },
            direction: Vector3D { x: 1.0, y: -1.0, z: 0.0 }
        };

        let result = plane.find_intersection(&ray);
        assert_eq!(2.0, result);
    }
}
