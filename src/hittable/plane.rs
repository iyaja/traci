use crate::hittable::aabb::AABB;
use crate::hittable::bvh::BoundingBox;
use crate::hittable::{HitRecord, Hittable};
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Plane {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if r.direction.dot(&self.normal) == 0.0 {
            return None;
        }

        let root = (self.point - r.origin).dot(&self.normal) / r.direction.dot(&self.normal);
        if root < t_min || root > t_max {
            return None;
        }

        let p = r.at(root);

        let rec = HitRecord {
            t: root,
            point: p,
            normal: self.normal,
            material: self.material,
        };

        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        None
    }
}

impl BoundingBox for Plane {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        None
    }
}
