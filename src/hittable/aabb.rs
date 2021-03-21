use crate::hittable::bvh;
use crate::ray::Ray;
use crate::vec3::*;

use min_max::*;
use std::cmp::{max, min};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn surrounding_box(a: AABB, b: AABB) -> AABB {
        let small = Point3::new(
            if a.min.x < b.min.x { a.min.x } else { b.min.x },
            if a.min.y < b.min.y { a.min.y } else { b.min.y },
            if a.min.z < b.min.z { a.min.z } else { b.min.z },
        );

        let big = Point3::new(
            if a.max.x > b.max.x { a.max.x } else { b.max.x },
            if a.max.y > b.max.y { a.max.y } else { b.max.y },
            if a.max.z > b.max.z { a.max.z } else { b.max.z },
        );

        AABB::new(small, big)
    }

    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
