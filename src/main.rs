use image::error::ImageError;
use indicatif::ProgressBar;
use rand::Rng;

mod camera;
mod hittable;
mod material;
mod ray;
mod texture;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use material::Material;
use ray::Ray;
use texture::Texture;
use vec3::Vec3;
use Material::*;
use Texture::*;

fn main() -> Result<(), ImageError> {
    let width = 1200;
    let height = 800;
    let num_samples = 10;

    let start = std::time::Instant::now();
    let image = generate_image(height, width, num_samples);
    println!(
        "Generated image in {:.2} seconds",
        start.elapsed().as_secs_f32()
    );

    write_image(image.buffer(), height, "output.png")?;

    Ok(())
}

struct ImageBuilder {
    buffer: Vec<Vec3>,
    num_samples: u32,
    width: usize,
}

impl ImageBuilder {
    fn new(height: usize, width: usize) -> Self {
        Self {
            buffer: vec![Vec3::default(); height * width],
            num_samples: 0,
            width,
        }
    }

    fn update(&mut self, x: usize, y: usize, col: Vec3) {
        let i = y * self.width + x;
        self.buffer[i] = col;
    }

    fn buffer(&self) -> &[Vec3] {
        &self.buffer
    }
}

fn generate_image(height: usize, width: usize, num_samples: u32) -> ImageBuilder {
    let world = populate_world();

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let upward = Vec3(0.0, 1.0, 0.0);
    let aspect_ratio = (width as f32) / (height as f32);
    let vfov = std::f32::consts::PI / 9.0;
    let aperture = 0.1;
    let focal_dist = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        upward,
        vfov,
        aspect_ratio,
        aperture,
        focal_dist,
    );

    let count = std::sync::atomic::AtomicU32::new(0);
    let bar = ProgressBar::new(100);

    let mut builder = ImageBuilder::new(height, width);

    let num_pixels = height * width;

    (0..num_pixels).for_each(|n| {
        let i = n % width;
        let j = height - (n / width) - 1;
        let mut rng = rand::thread_rng();
        let col_sum: Vec3 = (0..num_samples)
            .map(|_| {
                let x = (i as f32 + rng.gen::<f32>()) / width as f32;
                let y = (j as f32 + rng.gen::<f32>()) / height as f32;

                let ray = camera.get_ray(x, y);
                color(&ray, &world, 50)
            })
            .sum();

        let col = col_sum / (num_samples as f32);

        builder.update(i, height - j - 1, col);

        let prev_count = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let section = num_pixels / 100;
        if (prev_count + 1) % section as u32 == 0 {
            bar.inc(1)
        }
        if (prev_count + 1) % (num_pixels / 10) as u32 == 0 {
            write_image(builder.buffer(), height, "output.png").unwrap();
        }
    });

    bar.finish();

    builder
}

fn color(ray: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth == 0 {
        return background_color(&ray);
    };

    // Start t_range at non-zero value to prevent self-intersection
    match world.hit(ray, 0.001..std::f32::MAX) {
        Some(hit_record) => {
            let scattered = match hit_record.material.scatter(ray, &hit_record) {
                Some((new_ray, attenuation)) => attenuation * color(&new_ray, &world, depth - 1),
                None => Vec3(0.0, 0.0, 0.0),
            };

            hit_record.material.emit() + scattered
        }
        None => background_color(ray),
    }
}

fn background_color(_ray: &Ray) -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}

fn populate_world() -> HittableList {
    let mut world = HittableList {
        hittables: Vec::new(),
    };

    world.hittables.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian(Checkered(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0))),
    });

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

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

fn write_image(image: &[Vec3], height: usize, path: &str) -> Result<(), ImageError> {
    let width = image.len() / height;
    let mut buffer = Vec::with_capacity((height * width * 3) as usize);

    for pixel in image {
        buffer.push(to_rgb(pixel.0));
        buffer.push(to_rgb(pixel.1));
        buffer.push(to_rgb(pixel.2));
    }

    image::save_buffer_with_format(
        path,
        buffer.as_slice(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )?;

    Ok(())
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
