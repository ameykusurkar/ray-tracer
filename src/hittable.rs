use crate::vec3::Vec3;
use crate::ray::Ray;

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.dir.dot(ray.dir); 
    let b = 2.0 * ray.dir.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}
