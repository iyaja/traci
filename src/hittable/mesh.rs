use crate::hittable::triangle::{SimpleTriangle, Triangle};
use crate::hittable::{aabb::AABB, bvh::BVH};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;
use crate::Material;
use crate::Scene;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use obj::Obj;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::Path;
use tri_mesh::{mesh::Mesh, MeshBuilder};
use wavefront_obj;

#[derive(Clone)]
pub struct TriangleMesh {
    triangles: Vec<Triangle>,
}

impl TriangleMesh {
    /// Construct `TriangleMesh` from a `.obj` file
    pub fn from_file(file: &str, material: Material) -> Self {
        let mesh = MeshBuilder::new()
            .with_obj(read_to_string(file).unwrap())
            .build()
            .unwrap();
        let mut triangles = Vec::new();
        let pb = ProgressBar::new(mesh.no_faces() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.yellow}] ({eta})")
                .progress_chars("=> "),
        );

        for face_id in mesh.face_iter() {
            mesh.face_area(face_id);
            let (a, b, c) = mesh.face_positions(face_id);
            let p1 = Point3::new(a[0] as f32, a[1] as f32, a[2] as f32);
            let p2 = Point3::new(b[0] as f32, b[1] as f32, b[2] as f32);
            let p3 = Point3::new(c[0] as f32, c[1] as f32, c[2] as f32);

            let (ida, idb, idc) = mesh.face_vertices(face_id);
            let na = mesh.vertex_normal(ida);
            let nb = mesh.vertex_normal(idb);
            let nc = mesh.vertex_normal(idc);
            let n1 = Vec3::new(na[0] as f32, na[1] as f32, na[2] as f32);
            let n2 = Vec3::new(nb[0] as f32, nb[1] as f32, nb[2] as f32);
            let n3 = Vec3::new(nc[0] as f32, nc[1] as f32, nc[2] as f32);
            let triangle = Triangle::new(p1, p2, p3, n1, n2, n3, material);
            triangles.push(triangle);
            pb.inc(1);
        }

        TriangleMesh {
            triangles: triangles,
        }
    }

    /// Shift and scale all the triangles in the mesh
    pub fn shift_scale(&mut self, center: Point3, scale: f32) {
        for triangle in &mut self.triangles {
            triangle.p1 = (scale * triangle.p1) + center;
            triangle.p2 = (scale * triangle.p2) + center;
            triangle.p3 = (scale * triangle.p3) + center;
        }
    }

    /// Insert a `TriangleMesh` into a `World`
    pub fn insert_in(&self, world: &mut Scene, center: Point3, scale: f32) {
        for triangle in &self.triangles {
            let p1 = (scale * triangle.p1) + center;
            let p2 = (scale * triangle.p2) + center;
            let p3 = (scale * triangle.p3) + center;
            let shifted = Triangle::new(
                p1,
                p2,
                p3,
                triangle.n1,
                triangle.n2,
                triangle.n3,
                triangle.material,
            );
            world.add(shifted);
        }
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut closest_hit = None;

        for triangle in &self.triangles {
            let hit = triangle.hit(r, t_min, t_max);
            match hit {
                Some(rec) => {
                    if rec.t < closest {
                        closest = rec.t;
                        closest_hit = hit;
                    }
                }
                None => continue,
            }
        }
        closest_hit
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.triangles.is_empty() {
            return None;
        }

        let mut output_box: AABB = self.triangles[0].bounding_box(t0, t1)?;

        let first_box = true;
        for triangle in &self.triangles {
            match triangle.bounding_box(t0, t1) {
                Some(aabb) => {
                    output_box = AABB::surrounding_box(aabb, output_box);
                }
                None => return None,
            }
        }
        Some(output_box)
    }
}
