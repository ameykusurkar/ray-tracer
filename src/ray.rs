use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at_param(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}
