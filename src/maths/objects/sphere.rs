use maths::color::Color;
use maths::vector3d::Vector3D;
use maths::objects::object;
use maths::ray::Ray;

pub struct Sphere {
    pub center: Vector3D,
    pub radius: f64,
    pub color: Color
}

impl Sphere {
    pub fn new() -> Sphere {
        return Sphere {
            center: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
            color: Color { red: 0.5, green: 0.5, blue: 0.5, special: 0.0 }
        };
    }
}

impl object::Object for Sphere {
    fn get_normal_at(&self, point: &Vector3D) -> Vector3D {
        let x = &self.center.negative();
        println!("{:?}", x.x);
        return point.add(x);
    }

    fn find_intersection(&self, ray: &Ray) -> f64 {
        //See: https://web.archive.org/web/20160423072855/https://www.cs.unc.edu/~rademach/xroads-RT/RTarticle.html

        //Compute scalar distance from rayOrigin to circleOrigin using euclidean distance
        let e_o = Vector3D{
            x: self.center.x - ray.origin.x,
            y: self.center.y - ray.origin.y,
            z: self.center.z - ray.origin.z,
        };

        let v = e_o.dot_product(&ray.direction);
        let disc = (self.radius * self.radius) - ((e_o.dot_product(&e_o)) - (v * v));
        if disc < 0.0 {
            return -1.0;
        }

        let d = disc.sqrt();
        let dist = v - d;
        return dist;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maths::vector3d::Vector3D;
    use maths::ray::Ray;
    use maths::objects::object::Object;
    use maths::color::Color;

    #[test]
    fn get_normal_at_valid_input_should_return_correct_result() {
        let sphere = Sphere {
            center: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            radius: 1.0,
            color: Color { red: 0.5, green: 0.5, blue: 0.5, special: 0.0 }
        };

        let point = Vector3D { x: 2.0, y: 2.0, z: 2.0 };
        let result = sphere.get_normal_at(&point);

        assert_eq!(1.0, result.x);
        assert_eq!(1.0, result.y);
        assert_eq!(1.0, result.z);
    }

    #[test]
    fn find_intersection_valid_input_should_return_correct_result() {
        let sphere = Sphere {
            center: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            radius: 1.0,
            color: Color { red: 0.5, green: 0.5, blue: 0.5, special: 0.0 }
        };

        let ray = Ray {
            origin: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3D { x: 1.0, y: 1.0, z: 0.0 }
        };

        let result = sphere.find_intersection(&ray);

        assert!(result > 0.0);
    }
}
