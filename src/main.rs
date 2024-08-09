use clap::Parser;
use image::error::ImageError;
use rand::Rng;

use ray_tracer::Camera;
use ray_tracer::HittableList;
use ray_tracer::Material;
use ray_tracer::Quad;
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

    /// Ray samples per pixel
    #[arg(short, long, default_value_t = 10)]
    samples: u32,

    /// Bounces per ray
    #[arg(short, long, default_value_t = 10)]
    depth: u32,

    /// Which scene to render
    #[arg(long, value_enum, default_value_t = SceneArg::Spheres)]
    scene: SceneArg,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SceneArg {
    Spheres,
    Quads,
    CornellBox,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    let start = std::time::Instant::now();
    let scene = match args.scene {
        SceneArg::Quads => build_scene_quads(args.height, args.width),
        SceneArg::Spheres => build_scene_spheres(args.height, args.width),
        SceneArg::CornellBox => build_cornell_box(args.height, args.width),
    };
    let image = scene.render(args.height, args.width, args.samples, args.depth);
    println!(
        "Generated image in {:.2} seconds",
        start.elapsed().as_secs_f32()
    );

    write_image(&image, args.height, "output.png")
}

fn build_scene_spheres(height: u32, width: u32) -> Scene {
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

    let mut objects = HittableList::new();

    objects.push_sphere(Sphere {
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
                objects.push_sphere(Sphere {
                    center,
                    radius: 0.2,
                    material: random_material(),
                });
            }
        }
    }

    objects.push_sphere(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Texture::Constant(Vec3(0.4, 0.2, 0.1))),
    });
    objects.push_sphere(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    });
    objects.push_sphere(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Vec3(0.7, 0.6, 0.5), 0.0),
    });

    for s in generate_lights() {
        objects.push_sphere(s);
    }

    Scene { camera, objects }
}

fn build_scene_quads(height: u32, width: u32) -> Scene {
    let look_from = Vec3(0.0, 0.0, 9.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let upward = Vec3(0.0, 1.0, 0.0);

    let aspect_ratio = (width as f32) / (height as f32);
    let vfov = std::f32::consts::PI / 2.0;
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

    let mut objects = HittableList::new();

    //let left_red = Material::Lambertian(Texture::Constant(Vec3(1.0, 0.2, 0.2)));
    let left_red = Material::Metal(Vec3(1.0, 0.2, 0.2), 0.01);
    let back_green = Material::Lambertian(Texture::Constant(Vec3(0.2, 1.0, 0.1)));
    let right_blue = Material::Lambertian(Texture::Constant(Vec3(0.2, 0.2, 1.0)));
    //let upper_orange = Material::Lambertian(Texture::Constant(Vec3(1.0, 0.5, 0.0)));
    let upper_orange = Material::Light(Vec3(1.0, 1.0, 1.0));
    let lower_teal = Material::Lambertian(Texture::Constant(Vec3(0.2, 0.8, 0.8)));

    // Quads
    objects.push_quad(Quad::new(
        Vec3(-3.0, -2.0, 5.0),
        Vec3(0.0, 0.0, -4.0),
        Vec3(0.0, 4.0, 0.0),
        left_red,
    ));
    objects.push_quad(Quad::new(
        Vec3(-2.0, -2.0, 0.0),
        Vec3(4.0, 0.0, 0.0),
        Vec3(0.0, 4.0, 0.0),
        back_green,
    ));
    objects.push_quad(Quad::new(
        Vec3(3.0, -2.0, 1.0),
        Vec3(0.0, 0.0, 4.0),
        Vec3(0.0, 4.0, 0.0),
        right_blue,
    ));
    objects.push_quad(Quad::new(
        Vec3(-2.0, 3.0, 1.0),
        Vec3(4.0, 0.0, 0.0),
        Vec3(0.0, 0.0, 4.0),
        upper_orange,
    ));
    objects.push_quad(Quad::new(
        Vec3(-2.0, -3.0, 5.0),
        Vec3(4.0, 0.0, 0.0),
        Vec3(0.0, 0.0, -4.0),
        lower_teal,
    ));

    Scene { camera, objects }
}

fn build_cornell_box(height: u32, width: u32) -> Scene {
    let look_from = Vec3(278.0, 278.0, -800.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let upward = Vec3(0.0, 1.0, 0.0);

    let aspect_ratio = (width as f32) / (height as f32);
    let vfov = radians(40.0);
    let aperture = 0.1;
    let focal_dist = (look_at - look_from).magnitude();

    let camera = Camera::new(
        look_from,
        look_at,
        upward,
        vfov,
        aspect_ratio,
        aperture,
        focal_dist,
    );

    let mut objects = HittableList::new();

    let red = Material::Lambertian(Texture::Constant(Vec3(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Texture::Constant(Vec3(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Texture::Constant(Vec3(0.12, 0.45, 0.15)));

    objects.push_quad(Quad::new(
        Vec3(555.0, 0.0, 0.0),
        Vec3(0.0, 555.0, 0.0),
        Vec3(0.0, 0.0, 555.0),
        green,
    ));
    objects.push_quad(Quad::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 555.0, 0.0),
        Vec3(0.0, 0.0, 555.0),
        red,
    ));
    objects.push_quad(Quad::new(
        Vec3(343.0, 554.0, 332.0),
        Vec3(-130.0, 0.0, 0.0),
        Vec3(0.0, 0.0, -105.0),
        Material::Light(Vec3(15.0, 15.0, 15.0)),
    ));
    objects.push_quad(Quad::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(555.0, 0.0, 0.0),
        Vec3(0.0, 0.0, 555.0),
        white,
    ));
    objects.push_quad(Quad::new(
        Vec3(555.0, 555.0, 555.0),
        Vec3(-555.0, 0.0, 0.0),
        Vec3(0.0, 0.0, -555.0),
        white,
    ));
    objects.push_quad(Quad::new(
        Vec3(0.0, 0.0, 555.0),
        Vec3(555.0, 0.0, 0.0),
        Vec3(0.0, 555.0, 0.0),
        white,
    ));

    for s in box_sides(Vec3(130.0, 0.0, 65.0), Vec3(295.0, 165.0, 230.0), white) {
        objects.push_quad(s)
    }

    for s in box_sides(Vec3(265.0, 0.0, 295.0), Vec3(430.0, 330.0, 460.0), white) {
        objects.push_quad(s)
    }

    Scene { camera, objects }
}

fn generate_lights() -> Vec<Sphere> {
    let mut lights = Vec::new();
    for i in (-8..=8).step_by(4) {
        for j in (-8..=8).step_by(4) {
            lights.push(Sphere {
                center: Vec3(i as f32, 4.0, j as f32),
                radius: 1.0,
                material: Material::Light(Vec3(1.0, 1.0, 1.0)),
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

fn radians(deg: f32) -> f32 {
    std::f32::consts::PI * (deg / 180.0)
}

/// Returns the 3D box (six sides) that contains the opposite vertices a and b
fn box_sides(a: Vec3, b: Vec3, mat: Material) -> Vec<Quad> {
    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Vec3(a.0.min(b.0), a.1.min(b.1), a.2.min(b.2));
    let max = Vec3(a.0.max(b.0), a.1.max(b.1), a.2.max(b.2));

    let dx = Vec3(max.0 - min.0, 0.0, 0.0);
    let dy = Vec3(0.0, max.1 - min.1, 0.0);
    let dz = Vec3(0.0, 0.0, max.2 - min.2);

    vec![
        Quad::new(Vec3(min.0, min.1, max.2), dx, dy, mat), // front
        Quad::new(Vec3(max.0, min.1, max.2), -dz, dy, mat), // right
        Quad::new(Vec3(max.0, min.1, min.2), -dx, dy, mat), // back
        Quad::new(Vec3(min.0, min.1, min.2), dz, dy, mat), // left
        Quad::new(Vec3(min.0, max.1, max.2), dx, -dz, mat), // top
        Quad::new(Vec3(min.0, min.1, min.2), dx, dz, mat), // bottom
    ]
}
