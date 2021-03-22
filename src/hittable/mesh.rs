use crate::hittable::triangle::{SimpleTriangle, Triangle};
use crate::hittable::{aabb::AABB, bvh::BVH};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;
use crate::Material;
use crate::Scene;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use obj::Obj;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tri_mesh::{mesh::Mesh, MeshBuilder};
use wavefront_obj;

#[derive(Clone)]
pub struct TriangleMesh {
    bvh: Box<dyn Hittable>,
    triangles: Vec<SimpleTriangle>,
}

impl TriangleMesh {
    // pub fn from_wavefront(file: &str, material: Material) -> Self {
    //     let mut triangles: Vec<Triangle> = Vec::new();
    //     let objstr: String = std::fs::read_to_string(file).unwrap();
    //     let objects = wavefront_obj::obj::parse(objstr).unwrap().objects;

    //     for object in objects {
    //         for geo in object.geometry {
    //             for shape in geo.shapes {
    //                 match shape.primitive {
    //                     wavefront_obj::obj::Primitive::Triangle(vtn_a, vtn_b, vtn_c) => {
    //                         let (idx_a, _, _) = vtn_a;
    //                         let (idx_b, _, _) = vtn_b;
    //                         let (idx_c, _, _) = vtn_c;
    //                         let a = object.vertices[idx_a];
    //                         let b = object.vertices[idx_b];
    //                         let c = object.vertices[idx_c];

    //                         let p1 = Point3::new(a.x as f32, a.y as f32, a.z as f32);
    //                         let p2 = Point3::new(b.x as f32, b.y as f32, b.z as f32);
    //                         let p3 = Point3::new(c.x as f32, b.y as f32, b.z as f32);

    //                         triangles.push(Triangle::new(p1, p2, p3, material));
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }

    //     TriangleMesh {
    //         triangles: triangles,
    //     }
    // }

    pub fn from_obj(file: &str, material: Material) -> Self {
        // let mut triangles: Vec<Box<dyn Hittable>> = Vec::new();
        let mut triangles: Vec<SimpleTriangle> = Vec::new();
        let normals: Vec<Vec3> = Vec::new();
        let path = Path::new(file);
        let objdata: Obj = Obj::load(path).unwrap();

        for object in objdata.data.objects {
            for group in object.groups {
                for poly in group.polys {
                    let obj::SimplePolygon(idxs) = poly;
                    let obj::IndexTuple(pos_a, _, norm_a) = idxs[0];
                    let obj::IndexTuple(pos_b, _, norm_b) = idxs[1];
                    let obj::IndexTuple(pos_c, _, norm_c) = idxs[2];
                    let a = objdata.data.position[pos_a];
                    let b = objdata.data.position[pos_b];
                    let c = objdata.data.position[pos_c];

                    // let na = objdata.data.normal[norm_a.unwrap()];
                    // let nb = objdata.data.normal[norm_b.unwrap()];
                    // let nc = objdata.data.normal[norm_c.unwrap()];

                    let p1 = Point3::new(a[0], a[1], a[2]);
                    let p2 = Point3::new(b[0], b[1], b[2]);
                    let p3 = Point3::new(c[0], c[1], c[2]);

                    // let n1 = Point3::new(na[0], na[1], na[2]);
                    // let n2 = Point3::new(nb[0], nb[1], nb[2]);
                    // let n3 = Point3::new(nc[0], nc[1], nc[2]);

                    // let triangle = Triangle::raw(p1, p2, p3, n1, n2, n3, material);
                    let triangle = SimpleTriangle::new(p1, p2, p3, material);
                    triangles.push(triangle);
                }
            }
        }

        let mut hittables = Vec::new();
        for triangle in triangles.clone() {
            hittables.push(Box::new(triangle) as Box<dyn Hittable>);
        }
        let bvh = BVH::build(hittables, 0.0, 0.0);

        TriangleMesh {
            bvh: bvh,
            triangles: triangles,
        }
    }

    // pub fn from_file(file: &str, material: Material) -> Self {
    //     let mesh = MeshBuilder::new()
    //         .with_obj(file.to_string())
    //         .build()
    //         .unwrap();
    //     let mut triangles = Vec::new();

    //     let pb = ProgressBar::new(mesh.no_faces() as u64);
    //     pb.set_style(
    //         ProgressStyle::default_bar()
    //             .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
    //             .progress_chars("=> "),
    //     );

    //     for face_id in mesh.face_iter() {
    //         mesh.face_area(face_id);
    //         let (a, b, c) = mesh.face_positions(face_id);
    //         let p1 = Point3::new(a[0] as f32, a[1] as f32, a[2] as f32);
    //         let p2 = Point3::new(b[0] as f32, b[1] as f32, b[2] as f32);
    //         let p3 = Point3::new(c[0] as f32, c[1] as f32, c[2] as f32);

    //         let (ida, idb, idc) = mesh.face_vertices(face_id);
    //         let na = mesh.vertex_normal(ida);
    //         let nb = mesh.vertex_normal(idb);
    //         let nc = mesh.vertex_normal(idc);
    //         let n1 = Vec3::new(na[0] as f32, na[1] as f32, na[2] as f32);
    //         let n2 = Vec3::new(nb[0] as f32, nb[1] as f32, nb[2] as f32);
    //         let n3 = Vec3::new(nc[0] as f32, nc[1] as f32, nc[2] as f32);
    //         let triangle = Triangle::new(p1, p2, p3, n1, n2, n3, material);
    //         triangles.push(triangle);
    //         pb.inc(1);
    //     }

    //     TriangleMesh {
    //         mesh: mesh,
    //         triangles: triangles,
    //     }
    // }

    pub fn insert_in(&self, world: &mut Scene, center: Point3, scale: f32) {
        for triangle in &self.triangles {
            let p1 = (scale * triangle.p1) + center;
            let p2 = (scale * triangle.p2) + center;
            let p3 = (scale * triangle.p3) + center;
            let shifted = SimpleTriangle::new(p1, p2, p3, triangle.material);
            world.add(shifted);
        }
    }
}

// impl Hittable for TriangleMesh {
//     fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//         let mut closest = t_max;
//         let mut closest_hit = None;

//         for triangle in &self.triangles {
//             let hit = triangle.hit(r, t_min, t_max);
//             match hit {
//                 Some(rec) => {
//                     if rec.t < closest {
//                         closest = rec.t;
//                         closest_hit = hit;
//                     }
//                 }
//                 None => continue,
//             }
//         }
//         closest_hit
//     }

//     fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
//         if self.triangles.is_empty() {
//             return None;
//         }

//         let mut output_box: AABB = self.triangles[0].bounding_box(t0, t1)?;

//         let first_box = true;
//         for triangle in &self.triangles {
//             match triangle.bounding_box(t0, t1) {
//                 Some(aabb) => {
//                     output_box = AABB::surrounding_box(aabb, output_box);
//                 }
//                 None => return None,
//             }
//         }
//         Some(output_box)
//     }
// }
