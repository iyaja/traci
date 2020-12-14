use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }

        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }

        let p = r.at(root);
        let rec = HitRecord {
            t: root,
            point: p,
            normal: p - self.center / self.radius,
        };

        Some(rec)
    }
}
