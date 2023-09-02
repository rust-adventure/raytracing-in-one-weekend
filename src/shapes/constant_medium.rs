use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    textures::Texture,
};
use glam::DVec3;
use rand::prelude::*;
use std::{f64::consts::PI, ops::Range};

use super::Shapes;

pub struct ConstantMedium {
    boundary: Box<Shapes>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(
        boundary: Shapes,
        density: f64,
        texture: Texture,
    ) -> Self {
        Self {
            boundary: Box::new(boundary),
            neg_inv_density: -density.recip(),
            phase_function: Material::Isotropic {
                albedo: texture,
            },
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        let Some(mut rec1) = self
            .boundary
            .hit(ray, f64::NEG_INFINITY..f64::INFINITY)
        else {
            return None;
        };
        let Some(mut rec2) = self
            .boundary
            .hit(ray, (rec1.t + 0.0001)..f64::INFINITY)
        else {
            return None;
        };

        if rec1.t < interval.start {
            rec1.t = interval.start;
        }
        if rec2.t > interval.end {
            rec2.t = interval.end;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary =
            (rec2.t - rec1.t) * ray_length;
        let hit_distance =
            self.neg_inv_density * rng.gen::<f64>().log10();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let point = ray.at(t);

        let rec = HitRecord {
            point,
            normal: DVec3::new(1., 0., 0.), // arbitrary
            t,
            front_face: true, // also arbitrary
            material: self.phase_function.clone(),
            // Arbitrary u/v?
            u: 0.,
            v: 0.,
        };

        Some(rec)
    }
}
