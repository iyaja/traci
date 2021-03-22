use crate::hittable::bvh::BoundingBox;
use crate::hittable::sphere::Sphere;
use crate::hittable::{aabb::AABB, bvh::BVH, HitRecord, Hittable};
use crate::light::{Light, PointLight};
use crate::ray::Ray;

use crate::vec3::*;

#[derive(Clone)]
pub struct Scene {
    pub lights: Vec<PointLight>,
    objects: Vec<Box<dyn Hittable>>,
    background: (Color, Color),
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            lights: Vec::new(),
            objects: Vec::new(),
            background: (Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0)),
        }
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn accelerate(&mut self, t0: f32, t1: f32) {
        let mut nodes: Vec<Box<dyn Hittable>> = Vec::new();
        let mut extra: Vec<Box<dyn Hittable>> = Vec::new();
        for prim in self.objects.clone() {
            match prim.bounding_box(0.0, std::f32::MAX) {
                Some(_) => nodes.push(prim),
                None => extra.push(prim),
            }
        }
        println!("Found {} non-boundable objects", extra.len());
        println!("Adding {} hittables to BVH", nodes.len());
        let bvh = BVH::build(nodes, t0, t1);
        self.objects = Vec::new();
        self.objects.push(bvh);
        self.objects.append(&mut extra);
    }
}

impl Hittable for Scene {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut closest_hit = None;

        for object in &self.objects {
            let hit = object.hit(r, t_min, t_max);
            match hit {
                Some(rec) => {
                    if rec.t < closest {
                        closest = rec.t;
                        closest_hit = hit;
                    }
                }
                None => continue,
            }
        }
        closest_hit
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: AABB = self.objects[0].bounding_box(t0, t1)?;

        let first_box = true;
        for object in &self.objects {
            match object.bounding_box(t0, t1) {
                Some(aabb) => {
                    output_box = AABB::surrounding_box(aabb, output_box);
                }
                None => return None,
            }
        }
        Some(output_box)
    }
}
