use crate::hittable::bvh::BoundingBox;
use crate::hittable::{aabb::AABB, HitRecord, Hittable};
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let rec = HitRecord {
            t: root,
            point: p,
            normal: (p - self.center) / self.radius,
            material: self.material,
        };

        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - (self.radius * Vec3::new(1.0, 1.0, 1.0)),
            max: self.center + (self.radius * Vec3::new(1.0, 1.0, 1.0)),
        })
    }
}
