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
    Primitive(Box<dyn Hittable + Send + Sync>),
}

#[derive(Clone)]
pub struct BVH {
    node: Node,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable + Sync + Send>>, t0: f32, t1: f32) -> BVH {
        fn box_compare(
            t0: f32,
            t1: f32,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable + Sync + Send>, &Box<dyn Hittable + Sync + Send>) -> Ordering
        {
            move |a, b| {
                let a_bbox = a.bounding_box(t0, t1);
                let b_bbox = b.bounding_box(t0, t1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("could not compare bounding boxes")
                }
            }
        }

        fn axis_range(
            objects: &Vec<Box<dyn Hittable + Sync + Send>>,
            t0: f32,
            t1: f32,
            axis: usize,
        ) -> f32 {
            let (min, max) = objects
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    if let Some(aabb) = hit.bounding_box(t0, t1) {
                        (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|a| (a, axis_range(&objects, t0, t1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        objects.sort_unstable_by(box_compare(t0, t1, axis));
        let len = objects.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let prim = objects.pop().unwrap();
                if let Some(bbox) = prim.bounding_box(t0, t1) {
                    BVH {
                        node: Node::Primitive(prim),
                        bbox,
                    }
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
            _ => {
                let right = BVH::new(objects.drain(len / 2..).collect(), t0, t1);
                let left = BVH::new(objects, t0, t1);
                let bbox = AABB::surrounding_box(left.bbox, right.bbox);
                BVH {
                    node: Node::SubTree {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            match &self.node {
                Node::Primitive(prim) => prim.hit(r, t_min, t_max),
                Node::SubTree { left, right } => {
                    let lhit = left.hit(r, t_min, t_max);
                    if let Some(rec) = lhit {
                        t_max = rec.t;
                    }
                    if let Some(rec) = right.hit(r, t_min, t_max) {
                        Some(rec)
                    } else {
                        lhit
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
