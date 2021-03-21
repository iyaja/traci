use crate::hittable::aabb::AABB;
use crate::material::Material;
use crate::{ray::Ray, vec3::*};

pub mod aabb;
pub mod bvh;
pub mod mesh;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub trait Hittable: HittableClone {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable + Send + Sync>;
}

impl<T: 'static + Hittable + Send + Sync + Clone> HittableClone for T {
    fn clone_box(&self) -> Box<dyn Hittable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

impl Clone for Box<dyn Hittable + Send + Sync> {
    fn clone(&self) -> Box<dyn Hittable + Send + Sync> {
        self.clone_box()
    }
}
