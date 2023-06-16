mod camera;
mod hittable;
mod material;
mod ray;
mod texture;
mod vec3;

pub use camera::Camera;
pub use hittable::{Hittable, HittableList, Sphere};
pub use material::Material;
pub use ray::Ray;
pub use texture::Texture;
pub use vec3::Vec3;

pub fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return background_color();
    };

    // Start t_range at non-zero value to prevent self-intersection
    match world.hit(ray, 0.001..std::f32::MAX) {
        Some(hit_record) => {
            let scattered = match hit_record.material.scatter(ray, &hit_record) {
                Some((new_ray, attenuation)) => attenuation * color(&new_ray, world, depth - 1),
                None => Vec3(0.0, 0.0, 0.0),
            };

            hit_record.material.emit() + scattered
        }
        None => background_color(),
    }
}

const fn background_color() -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}
