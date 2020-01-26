use std::fs::File;
use std::path::Path;
use std::io::Write;
use rand::Rng;

mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, Sphere, HittableList};
use camera::Camera;
use material::Material;
use Material::*;

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
    world.hittables.push(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Lambertian(Vec3(0.1, 0.2, 0.5)),
    });
    world.hittables.push(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian(Vec3(0.8, 0.8, 0.0)),
    });
    world.hittables.push(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Dielectric(1.5),
    });
    // Negative radius to make the sphere look hollow
    world.hittables.push(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Dielectric(1.5),
    });
    world.hittables.push(Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal(Vec3(0.8, 0.6, 0.2), 0.0),
    });

    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let upward = Vec3(0.0, 1.0, 0.0);
    let aspect_ratio = (width as f32) / (height as f32);
    let vfov = std::f32::consts::PI / 10.0;
    let aperture = 1.0;

    let camera = Camera::new(look_from, look_at, upward, vfov, aspect_ratio, aperture);
    let mut rng = rand::thread_rng();

    for i in (0..height).rev() {
        for j in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let x = (j as f32 + rng.gen::<f32>()) / width as f32;
                let y = (i as f32 + rng.gen::<f32>())/ height as f32;

                let ray = camera.get_ray(x, y);
                col = col + color(&ray, &world, 50);
            }

            col = col / (num_samples as f32);

            result.push_str(
                &format!("{} {} {}\n", to_rgb(col.0), to_rgb(col.1), to_rgb(col.2))
            );
        }
    }

    result
}

fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 { return background_color(&ray) };

    // Start t_range at non-zero value to prevent self-intersection
    match world.hit(ray, 0.001..std::f32::MAX) {
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record) {
                Some((new_ray, attenuation)) =>  {
                    attenuation * color(&new_ray, &world, depth - 1)
                },
                None => Vec3(0.0, 0.0, 0.0),
            }
        },
        None => background_color(ray),
    }
}

fn background_color(ray: &Ray) -> Vec3 {
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.1 + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
