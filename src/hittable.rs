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
            let candidate_ts = [
                (-b - discriminant.sqrt()) / a,
                (-b + discriminant.sqrt()) / a,
            ];

            for &t in candidate_ts.iter() {
                if t_range.start < t && t < t_range.end {
                    let intersection = ray.at_param(t);
                    let normal = (intersection - self.center) / self.radius;
                    return Some(HitRecord {
                        intersection,
                        normal,
                        t,
                        material: self.material,
                    });
                }
            }
        }

        None
    }
}

pub struct HittableList {
    hittables: Vec<AnyHittable>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    pub fn push_sphere(&mut self, sphere: Sphere) {
        self.hittables.push(AnyHittable::Sphere(sphere));
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.hittables.push(AnyHittable::Quad(quad));
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

pub struct Quad {
    /// Corner of the quad
    pub q: Vec3,
    /// First vector adjacent to Q
    pub u: Vec3,
    /// Second vector adjacent to Q
    pub v: Vec3,
    pub material: Material,

    d: f32,
    normal: Vec3,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Material) -> Self {
        let n = Vec3::cross(u, v);
        let normal = n.normalize();
        let d = Vec3::dot(normal, q);
        let w = n / Vec3::dot(n, n);
        Self {
            q,
            u,
            v,
            material,
            d,
            normal,
            w,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let denom = Vec3::dot(self.normal, ray.dir);

        if denom.abs() < 1e-8 {
            // Ray is parallel to the plane
            return None;
        }

        let t = (self.d - Vec3::dot(self.normal, ray.origin)) / denom;

        if !(t_range.start < t && t < t_range.end) {
            return None;
        }

        let intersection = ray.at_param(t);
        let planar_hitpoint_vector = intersection - self.q;
        let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpoint_vector, self.v));
        let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpoint_vector));

        if !is_interior(alpha, beta) {
            return None;
        }

        // Can hit from either side of the plane
        let normal = if Vec3::dot(ray.dir, self.normal) < 0.0 {
            self.normal
        } else {
            -1.0 * self.normal
        };

        Some(HitRecord {
            intersection,
            normal,
            t,
            material: self.material,
        })
    }
}

fn is_interior(alpha: f32, beta: f32) -> bool {
    let unit_interval = 0.0..1.0;
    unit_interval.contains(&alpha) && unit_interval.contains(&beta)
}

enum AnyHittable {
    Sphere(Sphere),
    Quad(Quad),
}

impl Hittable for AnyHittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        match self {
            AnyHittable::Sphere(sphere) => sphere.hit(ray, t_range),
            AnyHittable::Quad(quad) => quad.hit(ray, t_range),
        }
    }
}
