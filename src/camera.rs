use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub bottom_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32) -> Self {
        let half_height = (vfov / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            bottom_left: Vec3(-half_width, -half_height, -1.0),
            horizontal: Vec3(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0 * half_height, 0.0),
            origin: Vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // TODO: Normalise the ray direction?
        Ray {
            origin: self.origin,
            dir: self.bottom_left + u * self.horizontal + v * self.vertical
        }
    }
}
