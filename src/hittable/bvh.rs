use crate::hittable::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::light::{Light, PointLight};
use crate::ray::Ray;
use crate::vec3::*;
use crate::Scene;
use crate::Sphere;

use rand::Rng;
use std::cmp::Ordering;

pub trait BoundingBox {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Clone)]
enum Node {
    SubTree { left: Box<BVH>, right: Box<BVH> },
    Primitive(Box<dyn Hittable>),
}

#[derive(Clone)]
pub struct BVH {
    // node: Node,
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVH {
    pub fn new(left: Box<dyn Hittable>, right: Box<dyn Hittable>, bbox: AABB) -> Self {
        BVH {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    pub fn build(mut objects: Vec<Box<dyn Hittable>>, t0: f32, t1: f32) -> Box<dyn Hittable> {
        let axis = rand::thread_rng().gen_range(0, 3);
        objects.sort_by(|a, b| {
            let lhit = a.bounding_box(0.0, 0.0).unwrap().min;
            let rhit = b.bounding_box(0.0, 0.0).unwrap().min;
            lhit[axis].partial_cmp(&rhit[axis]).unwrap()
        });

        match objects.len() {
            0 => panic!("cannot build BVH with no objects"),
            1 => objects.remove(0),
            2 => {
                let right = objects.remove(1);
                let left = objects.remove(0);
                let lbbox = left.bounding_box(t0, t1).unwrap();
                let rbbox = right.bounding_box(t0, t1).unwrap();
                // let bbox = AABB::surrounding_box(lbbox, rbbox);
                let bbox = lbbox.around(&rbbox);
                Box::new(BVH::new(left, right, bbox))
            }
            _ => {
                let mut a = objects;
                let b = a.split_off(a.len() / 2);
                let left = Self::build(b, t0, t1);
                let right = Self::build(a, t0, t1);
                let lbbox = left.bounding_box(t0, t1).unwrap();
                let rbbox = right.bounding_box(t0, t1).unwrap();
                // let bbox = AABB::surrounding_box(lbbox, rbbox);
                let bbox = lbbox.around(&rbbox);
                Box::new(BVH::new(left, right, bbox))
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let lhit = self.left.hit(r, t_min, t_max);
            let rhit = self.right.hit(r, t_min, t_max);
            match (lhit, rhit) {
                (None, None) => None,
                (None, Some(rrec)) => Some(rrec),
                (Some(lrec), None) => Some(lrec),
                (Some(lrec), Some(rrec)) => {
                    if lrec.t < rrec.t {
                        Some(lrec)
                    } else {
                        Some(rrec)
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}

#[derive(Copy, Clone)]
struct Null {}

impl Hittable for Null {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        None
    }
}
