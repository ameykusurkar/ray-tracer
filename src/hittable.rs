use std::ops::Range;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub intersection: Vec3,
    pub normal: Vec3,
    pub t: f32, // param for the incident ray
    pub material: Material,
}

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
    pub material: Material,
}

pub struct HittableList {
    // TODO: Allow this to be a vector of Hittables
    pub hittables: Vec<Sphere>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        // In theory, b = 2 * ray.dir.dot(oc). Since the 2 cancels out with
        // other terms in the quadratic formula, we only need to compute half_b.
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(ray.dir);
        let half_b = ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Either 0 roots, or 1 root (tangent)
        if discriminant <= 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the closest t in the range
        let mut t = (-half_b - discriminant_sqrt) / a;
        if t < t_range.start || t_range.end < t {
            t = (-half_b + discriminant_sqrt) / a;
            if t < t_range.start || t_range.end < t {
                return None;
            }
        }

        let intersection = ray.at_param(t);
        let normal = (intersection - self.center) / self.radius;
        Some(HitRecord {
            intersection,
            normal,
            t,
            material: self.material,
        })
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_t = t_range.end;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, t_range.start..closest_t) {
                closest_t = hit_record.t;
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }
}
