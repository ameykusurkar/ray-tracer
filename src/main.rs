use std::fs::File;
use std::path::Path;
use std::io::Write;
use rand::Rng;

mod vec3;
mod ray;
mod hittable;
mod camera;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, Sphere, HittableList};
use camera::Camera;

fn main() -> Result<(), std::io::Error> {
    let width = 200;
    let height = 100;
    let num_samples = 50;

    let content = create_file_content(height, width, num_samples);

    let path = Path::new("output.ppm");
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_file_content(height: i32, width: i32, num_samples: i32) -> String {
    let mut result = String::new();

    result.push_str(&format!("P3\n{} {}\n255\n", width, height));

    let mut world = HittableList {hittables: Vec::new()};
    world.hittables.push(Sphere {center: Vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.hittables.push(Sphere {center: Vec3(0.0, -100.5, -1.0), radius: 100.0});

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for i in (0..height).rev() {
        for j in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let x = (j as f32 + rng.gen::<f32>()) / width as f32;
                let y = (i as f32 + rng.gen::<f32>())/ height as f32;

                let ray = camera.get_ray(x, y);
                col = col + color(&ray, &world);
            }

            col = col / (num_samples as f32);

            result.push_str(
                &format!("{} {} {}\n", to_rgb(col.0), to_rgb(col.1), to_rgb(col.2))
            );
        }
    }

    result
}

fn color(ray: &Ray, world: &HittableList) -> Vec3 {
    // Start t_range at non-zero value to prevent self-intersection
    match world.hit(ray, 0.001..std::f32::MAX) {
        Some(hit_record) => {
            let dir = hit_record.normal + Vec3::random_in_unit_sphere();
            0.5 * color(&Ray {origin: hit_record.intersection, dir}, &world)
        },
        None => {
            let unit_dir = ray.dir.normalize();
            let t = 0.5 * (unit_dir.1 + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        },
    }
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
