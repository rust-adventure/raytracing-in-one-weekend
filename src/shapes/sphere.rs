use std::ops::Range;

use glam::DVec3;

use crate::{
    hittable::{aabb::Aabb, HitRecord, Hittable},
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
    bbox: Aabb,
}

impl Sphere {
    pub fn new(
        center: DVec3,
        radius: f64,
        material: Material,
    ) -> Self {
        let rvec = DVec3::splat(radius);
        let bbox =
            Aabb::from((center - rvec, center + rvec));

        Self {
            center,
            radius,
            material,
            move_to: None,
            bbox,
        }
    }
    pub fn with_move_to(mut self, to: DVec3) -> Self {
        let rvec = DVec3::splat(self.radius);
        let box1 = Aabb::from((
            self.center - rvec,
            self.center + rvec,
        ));
        let box2 = Aabb::from((to - rvec, to + rvec));
        let bbox = Aabb::from((box1, box2));
        // the raytracing series has this odd constructor that pre-calculates
        // a value, then stores it.
        self.move_to = Some(to - self.center);
        self.bbox = bbox;
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

    fn bounding_box(&self) -> crate::hittable::aabb::Aabb {
        self.bbox.clone()
    }
}
