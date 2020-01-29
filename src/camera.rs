use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    bottom_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,

    // Unit vectors on the focal plane
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, upward: Vec3,
               vfov: f32, aspect_ratio: f32,
               aperture: f32, focal_dist: f32) -> Self {
        let look_dir_opposite = look_from - look_at;

        // The screen will be a plane orthogonal to the direction we are looking
        // in, with the point we are looking from as the origin. `-w` is the unit
        // vector in the direction we are looking, and `u`, `v` are the pair of
        // unit vectors that define the horizontal and vertical directions in the
        // "looking plane" respectively.
        let w = look_dir_opposite.normalize();
        let u = upward.cross(w).normalize();
        let v = w.cross(u);

        let half_height = (vfov / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        Camera {
            bottom_left:
                look_from - focal_dist * (half_width * u + half_height * v + w),
            horizontal: 2.0 * focal_dist * half_width * u,
            vertical: 2.0 * focal_dist * half_height * v,
            origin: look_from,
            lens_radius: aperture / 2.0,
            u, v,
        }
    }

    fn point_on_focal_plane(&self, x: f32, y: f32) -> Vec3 {
        self.bottom_left + x * self.horizontal + y * self.vertical
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let random_in_lens = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = random_in_lens.0 * self.u + random_in_lens.1 * self.v;
        let ray_origin = self.origin + offset;

        // TODO: Normalise the ray direction?
        Ray {
            origin: ray_origin,
            dir: self.point_on_focal_plane(x, y) - ray_origin,
        }
    }
}
