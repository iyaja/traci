#![allow(
    dead_code,
    unused_imports,
    unused_assignments,
    unused_variables,
    non_upper_case_globals
)]

#[macro_use]
extern crate approx;
extern crate glam;
extern crate image;
extern crate nalgebra as na;

mod camera;
mod hittable;
mod light;
mod material;
mod ray;
mod scene;
mod vec3;

use camera::{Camera, OrthographicCamera, PerspectiveCamera};
use hittable::mesh::TriangleMesh;
use hittable::plane::Plane;
use hittable::sphere::Sphere;
use hittable::triangle::Triangle;
use hittable::{bvh::BVH, HitRecord, Hittable};
use light::{Light, PointLight};
use material::{Material, Scatter};
use ray::Ray;
use scene::Scene;
use vec3::*;

use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rand::prelude::*;
use rand_distr::{Distribution, Uniform};
use rayon::prelude::*;

fn ray_color(ray: Ray, world: &Scene, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit = world.hit(ray, 0.001, std::f32::MAX);
    match hit {
        Some(rec) => {
            let scatter = rec.material.scatter(ray, rec);
            return ray_color(scatter.ray, world, depth - 1).component_mul(&scatter.attenuation);
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            return ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0));
        }
    }
}

fn ray_color_phong(ray: Ray, world: &Scene, depth: i32) -> Color {
    let hit = world.hit(ray, 0.001, std::f32::MAX);

    match hit {
        Some(rec) => {
            let scatter = rec.material.scatter(ray, rec);
            let ambient = scatter.attenuation;
            let mut specular = Color::new(0.0, 0.0, 0.0);
            let mut diffuse = Color::new(0.0, 0.0, 0.0);

            let ambient_coeff: f32 = 0.3;
            let mut diffuse_coeff: f32 = 0.7;
            let mut specular_coeff: f32 = 0.6;

            for light in &world.lights {
                let light_vector = (rec.point - light.position).normalize();

                diffuse += rec.normal.dot(&light_vector).max(0.0)
                    * scatter.attenuation.component_mul(&light.color);
                let reflected = 2.0 * (light_vector.dot(&rec.normal)) * rec.normal - light_vector;
                specular += ray
                    .direction
                    .dot(&reflected.normalize())
                    .max(0.0)
                    .min(1.0)
                    .powi(40)
                    * scatter.attenuation.component_mul(&light.color);
            }

            for light in &world.lights {
                let shadow_ray = Ray::new(rec.point, rec.point - light.position);
                let shadow_hit = world.hit(shadow_ray, 0.001, std::f32::MAX);
                if let Some(_) = shadow_hit {
                    diffuse_coeff = 0.0;
                    specular_coeff = 0.0;
                }
            }
            // println!("{}", specular);
            return ambient_coeff * ambient + diffuse_coeff * diffuse;
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            return ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0));
            // return Color::new(0.0, 0.0, 0.0);
        }
    }
}

fn main() {
    // TODO: convert to clap args
    //  Image parameters
    const aspect_ratio: f32 = 1.0; // 16.0 / 9.0;
    const image_width: u32 = 400;
    const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    const samples_per_pixel: i32 = 10;
    const max_depth: i32 = 50;

    // Camera parameters
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    // Alternate viewpoint
    // let lookfrom = Point3::new(5.5, 0.5, -1.0);
    // let lookat = Point3::new(0.0, 0.0, -8.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov = 27.5;
    let focal_length = 10.0;
    let aperture = 0.1;

    // Setup main objects used for rendering
    let cam = PerspectiveCamera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focal_length,
    );
    let mut img = ImageBuffer::new(image_width, image_height);
    let mut world: Scene = Scene::new();

    // Scene parameters
    // let num_spheres = 30;
    // let radius_min = 0.0;
    // let radius_max = 0.0;
    // let x_min = 0.0;
    // let x_max = 0.0;
    // let y_min = 0.0;
    // let y_max = 0.0;

    // let world = random_scene(num_spheres);

    // Grid of spheres
    for nx in -50..50 {
        for ny in -50..50 {
            for nz in -50..50 {
                world.add(random_sphere(
                    nx as f32 / 10.0,
                    ny as f32 / 10.0,
                    -10.0 + (nz as f32 / 10.0),
                ))
            }
        }
    }

    // Teapot mesh
    // let mut mesh = TriangleMesh::from_file("objs/teapot.obj", random_material());
    // mesh.shift_scale(Point3::new(0.0, -2.0, -10.0), 0.5);
    // world.add(mesh);

    // Construct BVH and replace bounded objects
    world.accelerate(0.0, 0.0);

    world.add_light(PointLight::new(
        Point3::new(-10.0, -10.0, -10.0),
        Color::new(1.0, 1.3, 1.0),
    ));

    let pb = ProgressBar::new(image_height as u64 * image_width as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .progress_chars("=> "),
    );

    for (x, y, pix) in img.enumerate_pixels_mut() {
        let color_samples = [Color::new(0.0, 0.0, 0.0); samples_per_pixel as usize];
        let color: Color = color_samples
            .par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let u = (x as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (y as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                let r = cam.get_ray(u, v);
                ray_color_phong(r, &world, max_depth)
            })
            .sum();

        *pix = color.to_rgb_samples(samples_per_pixel);
        pb.inc(1);
    }

    image::imageops::flip_vertical_in_place(&mut img);
    img.save("images/out.png").unwrap();
}

fn random_sphere(x: f32, y: f32, z: f32) -> Sphere {
    let mut rng = rand::thread_rng();
    let random_float = Uniform::new_inclusive(0.0, 1.0);
    let random_radius = Uniform::new_inclusive(0.05, 0.1).sample(&mut rng);
    let random_albedo_r = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo_g = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo_b = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo = Color::new(random_albedo_r, random_albedo_g, random_albedo_b);
    let random_refractive_index = Uniform::new_inclusive(-1.5, 1.5).sample(&mut rng);
    let mat_picker = Uniform::new(0, 4).sample(&mut rng);
    let center = Point3::new(x, y, z);
    let sphere_material = match mat_picker {
        0 => {
            let albedo = random_albedo;
            Material::Lambertian { albedo }
        }
        1 => {
            let albedo = random_albedo;
            let fuzz = Uniform::new(0.0, 0.5).sample(&mut rand::thread_rng());
            Material::Metal { albedo, fuzz }
        }
        2 => Material::Dielectric {
            albedo: Color::new(1.0, 1.0, 1.0),
            refraction_index: 1.5,
        },
        _ => Material::Dielectric {
            albedo: random_albedo,
            refraction_index: random_refractive_index,
        },
    };
    Sphere::new(center, 0.05, sphere_material)
}

fn random_material() -> Material {
    let mut rng = rand::thread_rng();
    let mat_picker = Uniform::new(0, 4).sample(&mut rng);
    let random_albedo_r = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo_g = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo_b = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
    let random_albedo = Color::new(random_albedo_r, random_albedo_g, random_albedo_b);

    match mat_picker {
        0 => {
            let fuzz = Uniform::new(0.0, 0.5).sample(&mut rand::thread_rng());
            Material::Metal {
                albedo: random_albedo,
                fuzz,
            }
        }
        1 => Material::Dielectric {
            albedo: Color::new(1.0, 1.0, 1.0),
            refraction_index: 1.5,
        },
        _ => Material::Lambertian {
            albedo: random_albedo,
        },
    }
}
