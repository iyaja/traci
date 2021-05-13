use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub enum Light {
    Point(PointLight),
    Area(AreaLight),
}

impl Light {
    pub fn new(position: Point3, color: Color) -> Light {
        Light::Point(PointLight {
            position: position,
            color: color,
        })
    }
}

#[derive(Copy, Clone)]
pub struct PointLight {
    pub position: Point3,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point3, color: Color) -> PointLight {
        PointLight {
            position: position,
            color: color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AreaLight {
    pub position: Point3,
    pub color: Color,
}

impl AreaLight {
    pub fn new(position: Point3, color: Color) -> AreaLight {
        AreaLight {
            position: position,
            color: color,
        }
    }
}
