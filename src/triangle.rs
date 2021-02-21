use crate::hittable::{HitRecord, Hittable};
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p1: Point3,
    pub p2: Point3,
    pub p3: Point3,
    pub material: Material,
}

impl Triangle {
    pub fn new(p1: Point3, p2: Point3, p3: Point3, material: Material) -> Triangle {
        Triangle {
            p1,
            p2,
            p3,
            material,
        }
    }
}

impl Hittable for Triangle {
    // Uses the Möller–Trumbore intersection algorithm
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let epsilon = 1.0e-5_f32;

        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = r.direction.cross(&edge2);
        let a = edge1.dot(&h);

        // Check if ray is parallel to triangle
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.p1;
        let u = f * s.dot(&h);
        let q = s.cross(&edge1);
        let v = f * r.direction.dot(&q);

        if u < 0.0 || u > 1.0 {
            return None;
        };

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);

        // This means that there is a line intersection but not a ray intersection.
        if t < epsilon {
            return None;
        }

        let rec = HitRecord {
            t: t,
            point: r.at(t),
            normal: edge1.cross(&edge2).normalize(),
            material: self.material,
        };

        Some(rec)
    }
}
