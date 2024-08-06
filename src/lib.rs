use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

mod camera;
mod hittable;
mod material;
mod ray;
mod texture;
mod vec3;

use hittable::Hittable;

pub use camera::Camera;
pub use hittable::{HittableList, Sphere};
pub use ray::Ray;
pub use vec3::Vec3;
pub use material::Material;
pub use texture::Texture;

const BACKGROUND_COLOR: Vec3 = Vec3(0.0, 0.0, 0.0);

pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera,
}

impl Scene {
    pub fn render(&self, height: u32, width: u32, num_samples: u32) -> Vec<Vec3> {
        let count = std::sync::atomic::AtomicU32::new(0);
        let bar = ProgressBar::new(100);

        let num_pixels = height * width;
        let pixels = (0..num_pixels)
            .into_par_iter()
            .map(|n| {
                let i = n % width;
                let j = height - (n / width) - 1;
                let mut rng = rand::thread_rng();
                let col_sum: Vec3 = (0..num_samples)
                    .map(|_| {
                        let x = (i as f32 + rng.gen::<f32>()) / width as f32;
                        let y = (j as f32 + rng.gen::<f32>()) / height as f32;

                        let ray = &self.camera.get_ray(x, y);
                        color(&ray, &self.objects, 50)
                    })
                    .sum();

                let col = col_sum / (num_samples as f32);

                let prev_count = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let section = height * width / 100;
                if (prev_count + 1) % section == 0 {
                    bar.inc(1)
                }

                col
            })
            .collect();

        bar.finish();

        pixels
    }
}

fn color(ray: &Ray, objects: &HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return BACKGROUND_COLOR;
    };

    // Start t_range at non-zero value to prevent self-intersection
    if let Some(hit_record) = objects.hit(ray, 0.001..std::f32::MAX) {
        let scattered = match hit_record.material.scatter(ray, &hit_record) {
            Some((new_ray, attenuation)) => attenuation * color(&new_ray, objects, depth - 1),
            None => Vec3(0.0, 0.0, 0.0),
        };

        hit_record.material.emit() + scattered
    } else {
        BACKGROUND_COLOR
    }
}
