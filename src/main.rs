#[macro_use]
extern crate approx;
extern crate glam;
extern crate image;
extern crate nalgebra as na;

mod camera;
mod hittable;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use material::{Material, Scatter};
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
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

fn main() {
    // TODO: convert to clap args

    //  Image parameters
    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: u32 = 1920;
    const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    const samples_per_pixel: i32 = 1000;
    const max_depth: i32 = 100;

    // Camera parameters
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov = 27.5;
    let focal_length = 10.0;
    let aperture = 0.1;

    // Setup main objects used for rendering
    let mut cam = Camera::new(
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
    let num_spheres = 30;
    // let radius_min = 0.0;
    // let radius_max = 0.0;
    // let x_min = 0.0;
    // let x_max = 0.0;
    // let y_min = 0.0;
    // let y_max = 0.0;

    let world = random_scene(num_spheres);

    // let material_ground = Material::Lambertian {
    //     albedo: Color::new(0.8, 0.8, 0.0),
    // };
    // let material_center = Material::Lambertian {
    //     albedo: Color::new(0.0, 0.8, 0.8),
    // };
    // let material_left = Material::Metal {
    //     albedo: Color::new(0.8, 0.0, 0.8),
    //     fuzz: 1.0,
    // };
    // let material_right = Material::Dielectric {
    //     albedo: Color::new(1.0, 1.0, 1.0),
    //     refraction_index: 0.4,
    // };

    // world.add(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // ));
    // world.add(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center,
    // ));
    // world.add(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // ));
    // world.add(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // ));

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
                ray_color(r, &world, max_depth)
            })
            .sum();

        *pix = color.to_rgb_samples(samples_per_pixel);
        pb.inc(1);
    }

    image::imageops::flip_vertical_in_place(&mut img);
    img.save("images/test.png").unwrap();
}

fn random_scene(num_spheres: u32) -> Scene {
    let mut world = Scene::new();

    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    let sphere_range = (num_spheres as f32).sqrt() as i32;

    for a in -sphere_range..sphere_range {
        for b in -sphere_range..sphere_range {
            let random_float = Uniform::new_inclusive(0.0, 1.0);
            let random_radius = Uniform::new_inclusive(0.1, 0.4).sample(&mut rng);
            let random_x = Uniform::new_inclusive(11.0, 13.0).sample(&mut rng);
            let random_z = Uniform::new_inclusive(-5.0, 5.0).sample(&mut rng);
            let random_albedo_r = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
            let random_albedo_g = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
            let random_albedo_b = Uniform::new_inclusive(0.0, 1.0).sample(&mut rng);
            let random_albedo = Color::new(random_albedo_r, random_albedo_g, random_albedo_b);
            let random_refractive_index = Uniform::new_inclusive(-1.5, 1.5).sample(&mut rng);

            let mat_picker = Uniform::new(0, 4).sample(&mut rng);

            let center = Point3::new(
                a as f32 + 0.9 * random_float.sample(&mut rng),
                random_radius,
                b as f32 + 0.9 * random_float.sample(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
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
                world.add(Sphere::new(center, random_radius, sphere_material));
            }
        }
    }

    world
}
