use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
}

impl Material {
    pub fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3) {
        match self {
            Material::Lambertian(albedo) => {
                let ray = Ray {
                    origin: hit_record.intersection,
                    dir: hit_record.normal + Vec3::random_in_unit_sphere(),
                };
                (ray, *albedo)
            }
        }
    }
}
