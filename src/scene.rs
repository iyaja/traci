use crate::hittable::bvh::BoundingBox;
use crate::hittable::sphere::Sphere;
use crate::hittable::{aabb::AABB, HitRecord, Hittable};
use crate::light::{Light, PointLight};
use crate::ray::Ray;
use crate::vec3::*;

pub struct Scene {
    pub lights: Vec<PointLight>,
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
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

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
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
                    output_box = aabb.surrounding_box(output_box);
                }
                None => return None,
            }
        }
        Some(output_box)
    }
}
