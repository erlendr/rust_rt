use maths::vector3d::Vector3D;
use maths::ray::Ray;

pub trait Object {
    fn get_normal_at(&self, point: &Vector3D) -> Vector3D;
    fn find_intersection(&self, ray: &Ray) -> f64;
}
