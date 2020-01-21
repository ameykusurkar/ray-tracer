use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3) {
        match self {
            Material::Lambertian(albedo) => {
                let ray = Ray {
                    origin: hit_record.intersection,
                    dir: hit_record.normal + Vec3::random_in_unit_sphere(),
                };
                (ray, *albedo)
            }
            // TODO: Implement fuzziness
            Material::Metal(albedo) => {
                let ray = Ray {
                    origin: hit_record.intersection,
                    dir: reflect(ray.dir.normalize(), hit_record.normal),
                };
                (ray, *albedo)
            }
        }
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}
