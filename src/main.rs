use std::fs::File;
use std::path::Path;
use std::io::Write;

mod vec3;
mod ray;
mod hittable;

use vec3::Vec3;
use ray::Ray;
use hittable::hit_sphere;

fn main() -> Result<(), std::io::Error> {
    let width = 200;
    let height = 100;

    let content = create_file_content(height, width);

    let path = Path::new("output.ppm");
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_file_content(height: i32, width: i32) -> String {
    let mut result = String::new();

    result.push_str(&format!("P3\n{} {}\n255\n", width, height));

    let bottom_left = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for i in (0..height).rev() {
        for j in 0..width {
            let x = j as f32 / width as f32;
            let y = i as f32 / height as f32;

            let dir = bottom_left + (x * horizontal) + (y * vertical);
            let ray = Ray {origin, dir};
            let col = color(&ray);

            result.push_str(
                &format!("{} {} {}\n", to_rgb(col.0), to_rgb(col.1), to_rgb(col.2))
            );
        }
    }

    result
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3(1.0, 0.0, 0.0);
    }

    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.1 + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val) as u8
}
