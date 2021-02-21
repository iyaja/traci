use crate::ray::Ray;
use crate::vec3::*;

use rand_distr::{Distribution, UnitDisc};

// TODO: refactor both camera models into a single enum

#[derive(Copy, Clone, Debug)]
pub enum Camera {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera),
}

#[derive(Copy, Clone, Debug)]
pub struct OrthographicCamera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl PerspectiveCamera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focal_length: f32,
    ) -> PerspectiveCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focal_length * viewport_width * u;
        let vertical = focal_length * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * w;
        let lens_radius = aperture / 2.0;

        PerspectiveCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

impl OrthographicCamera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focal_length: f32,
    ) -> OrthographicCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focal_length * viewport_width * u;
        let vertical = focal_length * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * w;
        let lens_radius = aperture / 2.0;

        OrthographicCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: (s - 0.5) * self.horizontal + (t - 0.5) * self.vertical,
            direction: -self.horizontal.cross(&self.vertical).normalize(),
        }
    }
}
