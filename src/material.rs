use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
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
        }
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}
