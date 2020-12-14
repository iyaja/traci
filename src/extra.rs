
// #[derive(Copy, Clone)]
// pub struct Lambertian {
//     pub albedo: Vec3,
// }

// impl Scatter for Lambertian {
//     fn scatter(&self, _: Ray, hit: HitRecord) -> RayScatter {
//         let scatter_direction = hit.normal + random_in_unit_sphere();
//         let scattered = Ray::new(hit.point, scatter_direction);
//         let attenuation = self.albedo;
//         RayScatter::new(attenuation, scattered)
//     }
// }

// #[derive(Copy, Clone)]
// pub struct Metal {
//     pub albedo: Vec3,
//     pub fuzz: f32,
// }

// impl Scatter for Metal {
//     fn scatter(&self, ray: Ray, hit: HitRecord) -> RayScatter {
//         let reflected = ray.reflect(hit);
//         let attenuation = self.albedo;
//         let scattered = Ray::new(
//             reflected.origin,
//             reflected.direction + self.fuzz * random_in_unit_sphere(),
//         );
//         RayScatter::new(attenuation, reflected)
//     }
// }

// #[derive(Copy, Clone)]
// pub struct Dielectric {
//     pub refraction_index: f32,
// }

// impl Material for Dielectric {
//     pub fn scatter(self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Scatter {
//         // if the ray direction and hit normal are in the same half-sphere
//         let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit.n) > 0.0 {
//             (
//                 -hit.n,
//                 self.refraction_index,
//                 self.refraction_index * ray.direction.dot(hit.n) / ray.direction.length(),
//             )
//         } else {
//             (
//                 hit.n,
//                 1.0 / self.refraction_index,
//                 -ray.direction.dot(hit.n) / ray.direction.length(),
//             )
//         };

//         if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
//             let reflection_prob = schlick(cosine, self.refraction_index);
//             let out_dir = if rng.gen::<f32>() < reflection_prob {
//                 ray.direction.reflect(hit.n)
//             } else {
//                 refracted
//             };
//             Scatter::new(Vec3::ones(), Ray::new(hit.p, out_dir))
//         } else {
//             Scatter::new(Vec3::ones(), Ray::new(hit.p, ray.direction.reflect(hit.n)))
//         }
//     }
// }

// fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
//     // ni * sin(i) = nt * sin(t)
//     // sin(t) = sin(i) * (ni / nt)
//     let uv = v.make_unit_vector();
//     let dt = uv.dot(n);
//     let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
//     if discriminant > 0.0 {
//         let refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
//         Some(refracted)
//     } else {
//         None
//     }
// }

// fn schlick(cosine: f32, refraction_index: f32) -> f32 {
//     let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
//     r0 = r0 * r0;
//     r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
// }

// impl Material {
// pub fn lambertian(albedo: Vec3) -> Material {
//     Material::Lambertian(Lambertian { albedo })
// }

// pub fn metal(albedo: Vec3, fuzz: f32) -> Material {
//     Material::Metal(Metal { albedo, fuzz })
// }

// pub fn dielectric(refraction_index: f32) -> Material {
//     return Material::Dielectric(Dielectric { refraction_index });
// }
// }
