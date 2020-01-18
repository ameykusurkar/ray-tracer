use std::fs::File;
use std::path::Path;
use std::io::Write;

mod vec3;

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

    for i in 0..height {
        for j in 0..width {
            let col = vec3::Vec3(
                i as f32 / height as f32,
                j as f32 / width as f32,
                0.2,
            );
            result.push_str(
                &format!("{} {} {}\n", to_rgb(col.0), to_rgb(col.1), to_rgb(col.2))
            );
        }
    }

    result
}

fn to_rgb(val: f32) -> u8 {
    (255.0 * val) as u8
}
