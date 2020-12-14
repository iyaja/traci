use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, Color, Vec3};

#[derive(Copy, Clone)]
pub struct RayScatter {
    pub ray: Ray,
    pub attenuation: Color,
}

impl RayScatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> RayScatter {
        RayScatter { attenuation, ray }
    }
}

pub trait Scatter {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> RayScatter;
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

impl Scatter for Material {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> RayScatter {
        match hit.material {
            Material::Lambertian { albedo } => {
                let scatter_direction = hit.normal + random_in_unit_sphere();
                let scattered = Ray::new(hit.point, scatter_direction);
                let attenuation = albedo;
                RayScatter::new(attenuation, scattered)
            }

            Material::Metal { albedo, fuzz } => {
                let reflected = ray.reflect(hit);
                let attenuation = albedo;
                let scattered = Ray::new(
                    reflected.origin,
                    reflected.direction + fuzz * random_in_unit_sphere(),
                );
                RayScatter::new(attenuation, scattered)
            }

            Material::Dielectric { refraction_index } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refracted = ray.refract(hit, refraction_index);
                RayScatter::new(attenuation, refracted)
            }
        }
    }
}
