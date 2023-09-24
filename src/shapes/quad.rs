use std::{f64::consts::PI, ops::Range};

use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    textures::Texture,
};

pub struct Quad {
    Q: DVec3,
    u: DVec3,
    v: DVec3,
    material: Material,
    normal: DVec3,
    D: f64,
    w: DVec3,
}

impl Quad {
    pub fn new(
        Q: DVec3,
        u: DVec3,
        v: DVec3,
        material: Material,
    ) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let D = normal.dot(Q);
        let w = n / n.dot(n);
        Self {
            Q,
            u,
            v,
            material,
            normal,
            D,
            w,
        }
    }
    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if (a < 0.) || (1. < a) || (b < 0.) || (1. < b) {
            return None;
        }

        // a,b == u,v
        Some((a, b))
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }
        // Return false if the hit point parameter t is outside the ray interval.
        let t =
            (self.D - self.normal.dot(ray.origin)) / denom;
        if !interval.contains(&t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector: DVec3 =
            intersection - self.Q;
        let alpha =
            self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta =
            self.w.dot(self.u.cross(planar_hitpt_vector));

        let Some((u, v)) = Quad::is_interior(alpha, beta)
        else {
            return None;
        };
        // Ray hits the 2D shape; set the rest of the hit record and return true.
        // rec.t = t;
        // rec.p = intersection;
        // rec.mat = mat;
        // rec.set_face_normal(r, normal);

        // return true;

        let rec = HitRecord::with_face_normal(
            &self.material,
            intersection,
            self.normal,
            t,
            ray,
            u,
            v,
        );

        Some(rec)
    }
}
