extern crate approx;
extern crate nalgebra as na;

use image::{Rgb, Rgba};
use rand_distr::{Distribution, UnitBall};

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
