use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub bottom_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, upward: Vec3,
               vfov: f32, aspect_ratio: f32) -> Self {
        // The screen will be a plane orthogonal to the direction we are looking
        // in, with the point we are looking from as the origin. `-w` is the unit
        // vector in the direction we are looking, and `u`, `v` are the pair of
        // unit vectors that define the horizontal and vertical directions in the
        // "looking plane" respectively.
        let w = (look_from - look_at).normalize();
        let u = upward.cross(w).normalize();
        let v = w.cross(u);

        let half_height = (vfov / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            bottom_left: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // TODO: Normalise the ray direction?
        Ray {
            origin: self.origin,
            dir: self.bottom_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
