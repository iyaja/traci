use crate::material::Material;
use crate::{ray::Ray, vec3::*};

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
}
