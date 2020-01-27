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

    let image = generate_image(height, width, num_samples);
    write_to_ppm(&image, height)?;

    Ok(())
}

fn generate_image(height: i32, width: i32, num_samples: i32) -> Vec<Vec3> {
    let mut pixels = Vec::new();

    let world = populate_world();

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

            pixels.push(col);
        }
    }

    pixels
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

fn populate_world() -> HittableList {
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

    world
}

fn write_to_ppm(image: &Vec<Vec3>, height: i32) -> Result<(), std::io::Error> {
    let path = Path::new("output.ppm");
    let mut file = File::create(&path)?;

    let width = image.len() as i32 / height;

    file.write(&format!("P3\n{} {}\n255\n", width, height).as_bytes())?;

    for pixel in image {
        let (r, g, b) = (to_rgb(pixel.0), to_rgb(pixel.1), to_rgb(pixel.2));
        file.write(&format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
