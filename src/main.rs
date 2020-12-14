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
use material::{Material, Scatter, *};
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vec3::*;

use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use rayon::range::*;

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
            let unit_direction = ray.direction;
            let t = 0.5 * (unit_direction.y + 1.0);
            return ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0));
        }
    }
}

fn main() {
    // Declare camera and image constants
    // TODO: convert to clap args
    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: u32 = 300;
    const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    // const viewport_width: f32 = 4.0;
    // const viewport_height: f32 = viewport_width / aspect_ratio;
    // const focal_length: f32 = 1.0;
    const samples_per_pixel: i32 = 100;
    const max_depth: i32 = 50;

    let cam = Camera::new();
    let mut img = ImageBuffer::new(image_width, image_height);
    let mut world: Scene = Scene::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Dielectric {
        refraction_index: 1.5,
    };
    let material_left = Material::Dielectric {
        refraction_index: 1.5,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
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
                ray_color(r, &world, max_depth)
            })
            .sum();

        *pix = color.to_rgb_samples(samples_per_pixel);
        pb.inc(1);
    }

    image::imageops::flip_vertical_in_place(&mut img);
    img.save("images/test.png").unwrap();
}
