use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Texture {
    Constant(Vec3),
    Checkered(Vec3, Vec3),
}

impl Texture {
    pub fn value_at(self, p: Vec3) -> Vec3 {
        match self {
            Texture::Constant(col) => col,
            Texture::Checkered(col1, col2) => {
                let sines = (10.0 * p).map(f32::sin).reduce(std::ops::Mul::mul);
                if sines < 0.0 { col1 } else { col2 }
            }
        }
    }
}
