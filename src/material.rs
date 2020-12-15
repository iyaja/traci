use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, Color, Vec3, *};

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
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f32,
    },
    Dielectric {
        albedo: Color,
        refraction_index: f32,
    },
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

            Material::Dielectric {
                albedo,
                refraction_index,
            } => {
                let (normal, etai_over_etat) = if hit.normal.dot(&ray.direction) > 0.0 {
                    (-hit.normal, 1.0 / refraction_index)
                } else {
                    (hit.normal, refraction_index)
                };

                let unit_direction = ray.direction.normalize();
                let cos_theta = (-unit_direction).dot(&normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let out_ray = if etai_over_etat * sin_theta > 1.0 {
                    let reflected = reflect(unit_direction, normal);
                    Ray::new(hit.point, reflected)
                } else if rand::random::<f32>() < reflectance(cos_theta, etai_over_etat) {
                    let reflected = reflect(unit_direction, normal);
                    Ray::new(hit.point, reflected)
                } else {
                    let refracted = refract(unit_direction, normal, etai_over_etat);
                    Ray::new(hit.point, refracted)
                };

                // let attenuation = Color::new(1.0, 1.0, 1.0);
                let attenuation = albedo;
                // let refracted = ray.refract(hit, refraction_index);
                RayScatter::new(attenuation, out_ray)
            }
        }
    }
}

// Reflectance helper function
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_squared = r0.powi(2);
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}
