use rand::Rng;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian(albedo) => {
                let ray = Ray {
                    origin: hit_record.intersection,
                    dir: hit_record.normal + Vec3::random_in_unit_sphere(),
                };
                Some((ray, *albedo))
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = reflect(ray.dir.normalize(), hit_record.normal);
                let dir = reflected + *fuzz * Vec3::random_in_unit_sphere();
                if dir.dot(hit_record.normal) > 0.0 {
                    let ray = Ray { origin: hit_record.intersection, dir };
                    Some((ray, *albedo))
                } else {
                    // The ray has scattered below the surface
                    None
                }
            }
            Material::Dielectric(ref_idx) => {
                let incident = ray.dir.normalize();
                let ray_normal_dot = incident.dot(hit_record.normal);

                let (outward_normal, refract_ratio, cos_i) = if ray_normal_dot < 0.0 {
                    // Ray is coming from outside the surface
                    (hit_record.normal, 1.0 / ref_idx, -ray_normal_dot)
                } else {
                    (-1.0 * hit_record.normal, *ref_idx, ref_idx * ray_normal_dot)
                };

                let reflect_prob = schlick(cos_i, *ref_idx);
                let should_refract = rand::thread_rng().gen::<f32>() > reflect_prob;

                let dir = if should_refract {
                    match refract(incident, outward_normal, refract_ratio) {
                        Some(v) => v,
                        None => reflect(incident, outward_normal),
                    }
                } else {
                    reflect(incident, outward_normal)
                };

                let ray = Ray { origin: hit_record.intersection, dir };
                Some((ray, Vec3(1.0, 1.0, 1.0)))
            }
        }
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: Vec3, normal: Vec3, ratio: f32) -> Option<Vec3> {
    let cos_i = -incident.dot(normal);
    let cos_r_sq = 1.0 - (ratio * ratio) * (1.0 - (cos_i * cos_i));

    if cos_r_sq > 0.0 {
        Some(ratio * incident + (ratio * cos_i - cos_r_sq.sqrt()) * normal)
    } else {
        None
    }
}

// Approximates the probability of a ray being reflected,
// as opposed to being refracted
fn schlick(cos_i: f32, ref_idx: f32) -> f32 {
    let r0_sqrt = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (1.0 - r0) * (1.0 - cos_i).powf(5.0)
}
