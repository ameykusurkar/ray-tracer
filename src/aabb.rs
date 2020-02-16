use crate::vec3::Vec3;
use crate::ray::Ray;
// use std::ops::Range;

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        Aabb {
            min: box0.min.zip_with(box1.min, f32::min),
            max: box0.max.zip_with(box1.max, f32::max),
        }
    }
}
