use crate::{ray::Ray, vec3::*};

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub point: Point3,
    pub normal: Vec3,
}
