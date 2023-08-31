use std::ops::Range;

use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

// pub struct Sphere {
//     pub center: DVec3,
//     pub radius: f64,
//     pub material: Material,
// }

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Material,
    move_to: Option<DVec3>,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Default::default(),
            radius: Default::default(),
            material: Material::Lambertian {
                albedo: DVec3::new(0.0, 1., 1.),
            },
            move_to: None,
        }
    }
}
impl Sphere {
    pub fn new(
        center: DVec3,
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            center,
            radius,
            material,
            move_to: None,
        }
    }
    pub fn with_move_to(mut self, to: DVec3) -> Self {
        // the raytracing series has this odd constructor that pre-calculates
        // a value, then stores it.
        self.move_to = Some(to - self.center);
        self
    }
    fn center(&self, time: f64) -> DVec3 {
        match self.move_to {
            Some(center_vec) => {
                self.center + time * center_vec
            }
            None => self.center,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
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
        let outward_normal = (point - center) / self.radius;

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
