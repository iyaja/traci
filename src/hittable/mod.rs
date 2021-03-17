use crate::material::Material;
use crate::{ray::Ray, vec3::*};
use crate::hittable::aabb::AABB;

pub mod aabb;
pub mod bvh;
pub mod mesh;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub trait Hittable {
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
