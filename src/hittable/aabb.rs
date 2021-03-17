use crate::hittable::bvh;
use crate::ray::Ray;
use crate::vec3::*;

use min_max::*;
use std::cmp::{max, min};

pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
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

    pub fn surrounding_box(&self, other: AABB) -> AABB {
        let small = Point3::new(
            if self.min.x < other.min.x {
                self.min.x
            } else {
                other.min.x
            },
            if self.min.x < other.min.y {
                self.min.x
            } else {
                other.min.x
            },
            if self.min.x < other.min.y {
                self.min.x
            } else {
                other.min.x
            },
        );

        let big = Point3::new(
            if self.max.x < other.max.x {
                self.max.x
            } else {
                other.max.x
            },
            if self.max.x < other.max.y {
                self.max.x
            } else {
                other.max.x
            },
            if self.max.x < other.max.y {
                self.max.x
            } else {
                other.max.x
            },
        );

        return AABB::new(small, big);
    }
}
