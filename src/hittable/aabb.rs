use crate::hittable::bvh;
use crate::ray::Ray;
use crate::vec3::*;

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

    pub fn around(&self, box1: &Self) -> Self {
        let small = Vec3::new(
            self.min.x.min(box1.min.x),
            self.min.y.min(box1.min.y),
            self.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            self.max.x.max(box1.max.x),
            self.max.y.max(box1.max.y),
            self.max.z.max(box1.max.z),
        );
        Self {
            min: small,
            max: big,
        }
    }

    pub fn hit(&self, r: Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let t0 = (self.min[a] - r.origin[a]) * inv_d;
            let t1 = (self.max[a] - r.origin[a]) * inv_d;
            t_min = t_min.max(t0.min(t1));
            t_max = t_max.min(t0.max(t1));
        }
        t_max > t_min
    }
}
