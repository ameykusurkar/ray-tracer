use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    let width = 2000;
    let height = 1000;

    let content = create_file_content(height, width);

    let path = Path::new("output.ppm");
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file!"),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => {},
        Err(_) => {},
    };
}

fn create_file_content(height: i32, width: i32) -> String {
    let mut result = String::new();

    result.push_str("P3\n");
    result.push_str(&format!("{} {}\n", width, height));
    result.push_str("255\n");

    for i in 0..height {
        for j in 0..width {
            let r: i32 = (255.0 * i as f32 / height as f32) as i32;
            let g: i32 = (255.0 * j as f32 / width as f32) as i32;
            let b: i32 = (255.0 * 0.2) as i32;
            result.push_str(&format!("{} {} {}\n", r, g, b));
        }
    }

    result
}
