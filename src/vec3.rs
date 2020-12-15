extern crate approx;
extern crate nalgebra as na;

use image::{Rgb, Rgba};
use rand_distr::{Distribution, UnitBall, UnitDisc, UnitSphere};

// use glam::Vec3;
use na::Vector3;

pub type Vec3 = Vector3<f32>;
pub type Color = Vector3<f32>;
// pub type Point3 = na::Point3<f32>;
pub type Point3 = Vector3<f32>;

pub trait ColorVec {
    fn to_rgb(&self) -> Rgb<u8>;
    fn to_rgb_samples(&self, samples: i32) -> Rgb<u8>;
    fn to_rgba(&self) -> Rgba<u8>;
}

impl ColorVec for Vec3 {
    fn to_rgb(&self) -> Rgb<u8> {
        let r = self.x.sqrt() * 255.0;
        let g = self.y.sqrt() * 255.0;
        let b = self.z.sqrt() * 255.0;
        Rgb([r as u8, g as u8, b as u8])
    }

    fn to_rgb_samples(&self, samples: i32) -> Rgb<u8> {
        let scaled = self / samples as f32;
        let r = scaled.x.sqrt() * 255.0;
        let g = scaled.y.sqrt() * 255.0;
        let b = scaled.z.sqrt() * 255.0;
        Rgb([r as u8, g as u8, b as u8])
    }

    fn to_rgba(&self) -> Rgba<u8> {
        Rgba([self.x as u8, self.y as u8, self.z as u8, 1])
    }
}

pub trait PointVec {}

impl PointVec for Vec3 {}

pub fn random_in_unit_sphere() -> Vec3 {
    let [x, y, z] = UnitBall.sample(&mut rand::thread_rng());
    Vec3::new(x, y, z)
}

pub fn random_on_unit_sphere() -> Vec3 {
    let [x, y, z] = UnitSphere.sample(&mut rand::thread_rng());
    Vec3::new(x, y, z)
}

pub fn random_in_unit_hemisphere(normal: Vec3) -> Vec3 {
    let [x, y, z] = UnitSphere.sample(&mut rand::thread_rng());
    let in_unit_sphere = Vec3::new(x, y, z);
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disc() -> Vec3 {
    let [x, y] = UnitDisc.sample(&mut rand::thread_rng());
    Vec3::new(x, y, 0.0)
}

pub fn refract(direction: Vec3, surface_normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-direction).dot(&surface_normal);
    let r_out_perp = etai_over_etat * (direction + cos_theta * surface_normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * surface_normal;
    r_out_perp + r_out_parallel
}

pub fn reflect(direction: Vec3, surface_normal: Vec3) -> Vec3 {
    direction - 2.0 * direction.dot(&surface_normal) * surface_normal
}
