use clap::Parser;
use image::error::ImageError;
use rand::Rng;

use ray_tracer::Camera;
use ray_tracer::HittableList;
use ray_tracer::Material;
use ray_tracer::Scene;
use ray_tracer::Sphere;
use ray_tracer::Texture;
use ray_tracer::Vec3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the output image
    #[arg(short, long, default_value_t = 1200)]
    width: u32,

    /// Height of the output image
    #[arg(short, long, default_value_t = 800)]
    height: u32,

    /// Number of samples per pixel
    #[arg(short, long, default_value_t = 10)]
    samples: u32,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    let start = std::time::Instant::now();
    let image = generate_image(args.height, args.width, args.samples);
    println!(
        "Generated image in {:.2} seconds",
        start.elapsed().as_secs_f32()
    );

    write_image(&image, args.height, "output.png")
}

fn generate_image(height: u32, width: u32, num_samples: u32) -> Vec<Vec3> {
    let objects = populate_world();

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

    let scene = Scene { objects, camera };
    scene.render(height, width, num_samples)
}

fn populate_world() -> HittableList {
    let mut world = HittableList {
        hittables: Vec::new(),
    };

    world.hittables.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Texture::Checkered(
            Vec3(0.0, 0.0, 0.0),
            Vec3(1.0, 1.0, 1.0),
        )),
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
        material: Material::Lambertian(Texture::Constant(Vec3(0.4, 0.2, 0.1))),
    });
    world.hittables.push(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    });
    world.hittables.push(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Vec3(0.7, 0.6, 0.5), 0.0),
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
                material: Material::Light,
            });
        }
    }

    lights
}

fn random_material() -> Material {
    let mut rng = rand::thread_rng();
    let choice = rng.gen::<f32>();

    if choice < 0.5 {
        Material::Lambertian(Texture::Constant(Vec3::random() * Vec3::random()))
    } else if choice < 0.75 {
        Material::Metal(0.5 * (Vec3::random() + 1.0), 0.5 * rng.gen::<f32>())
    } else {
        Material::Dielectric(1.5)
    }
}

fn write_image(image: &Vec<Vec3>, height: u32, path: &str) -> Result<(), ImageError> {
    let width = image.len() as u32 / height;
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
    )
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val.sqrt()) as u8
}
