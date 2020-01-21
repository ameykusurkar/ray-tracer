use std::ops::Range;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

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
    pub hittables: Vec<Sphere>
}

impl Hittable for Sphere {
    // In theory, b = 2 * dot(ray.dir, oc). However, this cancels out with
    // 2s in the quadratic formula.
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t_range.start < t && t < t_range.end {
                let intersection = ray.at_param(t);
                let normal = (intersection - self.center).normalize();
                return Some(HitRecord {intersection, normal, t, material: self.material});
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t_range.start < t && t < t_range.end {
                let intersection = ray.at_param(t);
                let normal = (intersection - self.center).normalize();
                return Some(HitRecord {intersection, normal, t, material: self.material});
            }
        }

        None
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
