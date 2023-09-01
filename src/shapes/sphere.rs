use std::{f64::consts::PI, ops::Range};

use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    textures::Texture,
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
                albedo: Texture::SolidColor(DVec3::new(
                    0.0, 1., 1.,
                )),
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
    fn get_sphere_uv(&self, p: DVec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;
        (u, v)
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
        let (u, v) = self.get_sphere_uv(outward_normal);

        let rec = HitRecord::with_face_normal(
            self.material.clone(),
            point,
            outward_normal,
            t,
            ray,
            u,
            v,
        );

        Some(rec)
    }
}
