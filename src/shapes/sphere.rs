use std::ops::Range;

use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c =
            oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !interval.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal =
            (point - self.center) / self.radius;

        let rec = HitRecord::with_face_normal(
            self.material.clone(),
            point,
            outward_normal,
            t,
            ray,
        );

        Some(rec)
    }
}
