use crate::vec3::*;

use rand;

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

fn test_scene() -> Scene {
    let mut world: Scene = Scene::new();

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.0, 0.8, 0.8),
    };
    let material_left = Material::Metal {
        albedo: Color::new(0.8, 0.0, 0.8),
        fuzz: 1.0,
    };
    let material_right = Material::Dielectric {
        albedo: Color::new(1.0, 1.0, 1.0),
        refraction_index: 0.4,
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
    // world.add(Plane::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     Vec3::new(0.0, 1.0, -1.0),
    //     material_center,
    // ));
    world.add(Triangle::new(
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(1.0, 0.0, -1.0),
        Point3::new(0.0, -1.0, -1.0),
        material_left,
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

    world
}

fn simple_scene() -> Scene {
    let mut world: Scene = Scene::new();
    world.add(Triangle::new(
        Point3::new(4.0, -1.0, -8.0),
        Point3::new(1.0, 1.5, -9.0),
        Point3::new(2.0, 0.0, -10.0),
        Material::Metal {
            albedo: Color::new(1.0, 0.0, 0.0),
            fuzz: 1.0,
        },
    ));

    world.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.1),
        Material::Metal {
            albedo: Color::new(1.0, 1.0, 0.0),
            fuzz: 1.0,
        },
    ));

    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -8.0),
        1.0,
        Material::Metal {
            albedo: Color::new(1.0, 0.0, 1.0),
            fuzz: 1.0,
        },
    ));

    world.add(Sphere::new(
        Point3::new(0.8, 0.7, -7.0),
        0.4,
        Material::Metal {
            albedo: Color::new(0.2, 0.2, 1.0),
            fuzz: 1.0,
        },
    ));
}
