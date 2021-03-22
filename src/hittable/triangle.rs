use crate::hittable::bvh::BoundingBox;
use crate::hittable::{aabb::AABB, HitRecord, Hittable};
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
/// Triangle without interpolated normals
pub struct SimpleTriangle {
    pub p1: Point3,
    pub p2: Point3,
    pub p3: Point3,
    pub material: Material,
}

impl SimpleTriangle {
    pub fn new(p1: Point3, p2: Point3, p3: Point3, material: Material) -> Self {
        SimpleTriangle {
            p1,
            p2,
            p3,
            material,
        }
    }
}

impl Hittable for SimpleTriangle {
    // Uses the Möller–Trumbore intersection algorithm
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let epsilon = 1.0e-5_f32;

        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = r.direction.cross(&edge2);
        let a = edge1.dot(&h);

        // Check if ray is parallel to triangle
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.p1;
        let u = f * s.dot(&h);
        let q = s.cross(&edge1);
        let v = f * r.direction.dot(&q);

        if u < 0.0 || u > 1.0 {
            return None;
        };

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);

        // This means that there is a line intersection but not a ray intersection.
        if t < epsilon {
            return None;
        }

        let rec = HitRecord {
            t: t,
            point: r.at(t),
            normal: edge1.cross(&edge2).normalize(),
            material: self.material,
        };

        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let max_x = [self.p1.x, self.p2.x, self.p3.x]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );
        let max_y = [self.p1.y, self.p2.y, self.p3.y]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );
        let max_z = [self.p1.z, self.p2.z, self.p3.z]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );

        let min_x = [self.p1.x, self.p2.x, self.p3.x]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );
        let min_y = [self.p1.y, self.p2.y, self.p3.y]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );
        let min_z = [self.p1.z, self.p2.z, self.p3.z]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );

        Some(AABB {
            min: Point3::new(max_x, max_y, max_z),
            max: Point3::new(min_x, min_y, min_z),
        })
    }
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub p1: Point3,
    pub p2: Point3,
    pub p3: Point3,
    pub n1: Vec3,
    pub n2: Vec3,
    pub n3: Vec3,
    pub material: Material,
}

impl Triangle {
    /// Create new triangle by specifying points and vertex normals
    pub fn new(
        p1: Point3,
        p2: Point3,
        p3: Point3,
        n1: Vec3,
        n2: Vec3,
        n3: Vec3,
        material: Material,
    ) -> Triangle {
        Triangle {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            material,
        }
    }

    /// Compute barycentric coordinates w.r.t this triangle given a point
    fn barycentric(&self, p: Point3) -> (f32, f32, f32) {
        let v0: Vec3 = self.p2 - self.p1;
        let v1: Vec3 = self.p3 - self.p1;
        let v2: Vec3 = p - self.p1;
        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);
        let d20 = v2.dot(&v0);
        let d21 = v2.dot(&v1);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        (u, v, w)
    }

    /// Get interpolated normal if possible
    fn normal(&self, p: Point3) -> Vec3 {
        let (u, v, w) = self.barycentric(p);
        (u * self.n1) + (v * self.n2) + (w * self.n3)
    }
}

impl Hittable for Triangle {
    // Uses the Möller–Trumbore intersection algorithm with interpolated normals
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let epsilon = 1.0e-5_f32;

        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = r.direction.cross(&edge2);
        let a = edge1.dot(&h);

        // Check if ray is parallel to triangle
        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.p1;
        let u = f * s.dot(&h);
        let q = s.cross(&edge1);
        let v = f * r.direction.dot(&q);

        if u < 0.0 || u > 1.0 {
            return None;
        };

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);

        // This means that there is a line intersection but not a ray intersection.
        if t < epsilon {
            return None;
        }

        let point = r.at(t);

        let rec = HitRecord {
            t: t,
            point: point,
            normal: self.normal(point),
            material: self.material,
        };

        Some(rec)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let max_x = [self.p1.x, self.p2.x, self.p3.x]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );
        let max_y = [self.p1.y, self.p2.y, self.p3.y]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );
        let max_z = [self.p1.z, self.p2.z, self.p3.z]
            .iter()
            .fold(
                std::f32::MIN,
                |max, val| if val > &max { *val } else { max },
            );

        let min_x = [self.p1.x, self.p2.x, self.p3.x]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );
        let min_y = [self.p1.y, self.p2.y, self.p3.y]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );
        let min_z = [self.p1.z, self.p2.z, self.p3.z]
            .iter()
            .fold(
                std::f32::MAX,
                |min, val| if val < &min { min } else { *val },
            );

        Some(AABB {
            min: Point3::new(max_x, max_y, max_z),
            max: Point3::new(min_x, min_y, min_z),
        })
    }
}
