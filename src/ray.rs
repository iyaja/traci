use crate::hittable::HitRecord;
use crate::vec3::*;

use rand_distr::{Distribution, Uniform};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn reflect(&self, hit: HitRecord) -> Ray {
        Ray::new(
            hit.point,
            self.direction - 2.0 * self.direction.dot(&hit.normal) * hit.normal,
        )
    }

    pub fn refract(&self, hit: HitRecord, refraction_index: f32) -> Ray {
        let refraction_ratio = if self.direction.dot(&hit.normal) > 0.0 {
            refraction_index
        } else {
            1.0 / refraction_index
        };

        let unit_direction = self.direction.normalize();

        let cos_theta = if -unit_direction.dot(&hit.normal) < 1.0 {
            -unit_direction.dot(&hit.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let random_range = Uniform::new(0.0, 1.0);
        let random_float = random_range.sample(&mut rand::thread_rng());

        if refraction_ratio * sin_theta > 1.0
        // || self.reflactance(cos_theta, refraction_index) > random_float
        {
            return self.reflect(hit);
        }

        let r_out_perp = refraction_ratio * (unit_direction + cos_theta * hit.normal);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * hit.normal;
        Ray::new(hit.point, r_out_perp + r_out_parallel)
    }

    fn reflectance(&self, cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
    }
}
