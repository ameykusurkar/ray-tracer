use rand::Rng;
use indicatif::ProgressBar;
use image::error::ImageError;

mod vec3;
mod ray;
mod hittable;
mod camera;
mod material;
mod texture;

use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, Sphere, HittableList};
use camera::Camera;
use material::Material;
use Material::*;
use texture::Texture;
use Texture::*;

fn main() -> Result<(), ImageError> {
    let width = 1200;
    let height = 800;
    let num_samples = 10;

    let image = generate_image(height, width, num_samples);

    println!("Writing image to file...");
    let start = std::time::Instant::now();
    write_image(&image, height)?;
    println!("Done! Took {:.2} seconds", start.elapsed().as_secs_f32());

    Ok(())
}

fn generate_image(height: i32, width: i32, num_samples: i32) -> Vec<Vec3> {
    let world = populate_world();

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let upward = Vec3(0.0, 1.0, 0.0);
    let aspect_ratio = (width as f32) / (height as f32);
    let vfov = std::f32::consts::PI / 9.0;
    let aperture = 0.1;
    let focal_dist = 10.0;

    let camera = Camera::new(look_from, look_at, upward,
                             vfov, aspect_ratio, aperture, focal_dist);
    let mut rng = rand::thread_rng();

    let mut count = 0;
    let bar = ProgressBar::new(100);

    let pixels = generate_pixels(height, width, |i, j| {
        let col_sum: Vec3 = (0..num_samples).map(|_| {
            let x = (i as f32 + rng.gen::<f32>()) / width as f32;
            let y = (j as f32 + rng.gen::<f32>()) / height as f32;

            let ray = camera.get_ray(x, y);
            color(&ray, &world, 50)
        }).sum();

        let col = col_sum / (num_samples as f32);

        count += 1;
        let section = height * width / 100;
        if count % section == 0 { bar.inc(1) }

        col
    });

    bar.finish();

    pixels
}

fn generate_pixels(height: i32, width: i32, mut f: impl FnMut(i32, i32) -> Vec3) -> Vec<Vec3> {
    let mut pixels = Vec::with_capacity((height * width) as usize);

    for j in (0..height).rev() {
        for i in 0..width {
            pixels.push(f(i, j));
        }
    }

    pixels
}

fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 { return background_color(&ray) };

    // Start t_range at non-zero value to prevent self-intersection
    match world.hit(ray, 0.001..std::f32::MAX) {
        Some(hit_record) => {
            let scattered = match hit_record.material.scatter(ray, &hit_record) {
                Some((new_ray, attenuation)) =>  {
                    attenuation * color(&new_ray, &world, depth - 1)
                },
                None => Vec3(0.0, 0.0, 0.0),
            };

            hit_record.material.emit() + scattered
        },
        None => background_color(ray),
    }
}

fn background_color(_ray: &Ray) -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}

fn populate_world() -> HittableList {
    let mut world = HittableList {hittables: Vec::new()};

    world.hittables.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian(
            Checkered(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0))
        ),
    });

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (center - Vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                world.hittables.push(Sphere {
                    center,
                    radius: 0.2,
                    material: random_material(),
                });
            }
        }
    }

    world.hittables.push(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian(Constant(Vec3(0.4, 0.2, 0.1))),
    });
    world.hittables.push(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric(1.5),
    });
    world.hittables.push(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal(Vec3(0.7, 0.6, 0.5), 0.0),
    });

    world.hittables.append(&mut generate_lights());

    world
}

fn generate_lights() -> Vec<Sphere> {
    let mut lights = Vec::new();
    for i in (-8..=8).step_by(4) {
        for j in (-8..=8).step_by(4) {
            lights.push(Sphere {
                center: Vec3(i as f32, 4.0, j as f32),
                radius: 1.0,
                material: Light,
            });
        }
    }

    lights
}

fn random_material() -> Material {
    let mut rng = rand::thread_rng();
    let choice = rng.gen::<f32>();

    if choice < 0.5 {
        Lambertian(Constant(Vec3::random() * Vec3::random()))
    } else if choice < 0.75 {
        Metal(0.5 * (Vec3::random() + 1.0), 0.5 * rng.gen::<f32>())
    } else {
        Dielectric(1.5)
    }
}

fn write_image(image: &Vec<Vec3>, height: i32) -> Result<(), ImageError> {
    let width = image.len() as i32 / height;
    let mut buffer = Vec::with_capacity((height * width * 3) as usize);

    for pixel in image {
        buffer.push(to_rgb(pixel.0));
        buffer.push(to_rgb(pixel.1));
        buffer.push(to_rgb(pixel.2));
    }

    image::save_buffer("output.png",
                       buffer.as_slice(),
                       width as u32,
                       height as u32,
                       image::ColorType::Rgb8)?;

    Ok(())
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
