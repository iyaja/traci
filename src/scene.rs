use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::*;

use std::sync::Arc;

pub struct Scene {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    background: (Color, Color),
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            background: (Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0)),
        }
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, object: T) {
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for Scene {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        for object in &self.objects {
            let hit = object.hit(r, t_min, t_max);
            match hit {
                Some(rec) => return Some(rec),
                None => continue,
            }
        }
        None
    }
}