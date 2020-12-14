use crate::hittable::HitRecord;
use crate::vec3::*;

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
        Ray {
            origin: hit.point,
            direction: self.direction - 2.0 * self.direction.dot(&hit.normal) * hit.normal,
        }
    }

    pub fn refract(&self, hit: HitRecord, refraction_index: f32) -> Ray {
        let unit_direction = self.direction.normalize();
        let refraction_ratio = if unit_direction.dot(&hit.normal) > 0.0 {
            1.0 / refraction_index
        } else {
            refraction_index
        };

        let cos_theta = (-unit_direction).dot(&hit.normal);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        if refraction_ratio * sin_theta > 1.0 || self.reflactance(cos_theta, refraction_index) > 0.5
        {
            return self.reflect(hit);
        }

        let r_out_perp = refraction_ratio * (unit_direction + cos_theta * hit.normal);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * hit.normal;
        return Ray::new(hit.point, r_out_perp + r_out_parallel);
    }

    fn reflactance(&self, cosine: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
