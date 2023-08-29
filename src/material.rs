use std::ops::Neg;

use crate::{hittable::HitRecord, ray::Ray};
use glam::DVec3;

use rand::Rng;
use reflections::*;
mod reflections;
mod vectors;
use vectors::*;

#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}
pub struct Scattered {
    pub attenuation: DVec3,
    pub scattered: Ray,
}
impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<Scattered> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record
                    .normal
                    + random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction
                    .abs_diff_eq(DVec3::ZERO, 1e-8)
                {
                    scatter_direction = hit_record.normal;
                }

                Some(Scattered {
                    attenuation: *albedo,
                    scattered: Ray {
                        origin: hit_record.point,
                        direction: scatter_direction,
                    },
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected: DVec3 = reflect(
                    r_in.direction.normalize(),
                    hit_record.normal,
                );
                let scattered = Ray {
                    origin: hit_record.point,
                    direction: reflected
                        + *fuzz * random_unit_vector(),
                };
                // absorb any scatter that is below the surface
                if scattered
                    .direction
                    .dot(hit_record.normal)
                    > 0.
                {
                    Some(Scattered {
                        attenuation: *albedo,
                        scattered,
                    })
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let mut rng = rand::thread_rng();

                let attenuation = DVec3::splat(1.0);
                let refraction_ratio: f64 =
                    if hit_record.front_face {
                        index_of_refraction.recip()
                    } else {
                        *index_of_refraction
                    };

                let unit_direction =
                    r_in.direction.normalize();

                let cos_theta = unit_direction
                    .dot(hit_record.normal)
                    .neg()
                    .min(1.0);
                let sin_theta =
                    (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract =
                    refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract
                    || reflectance(
                        cos_theta,
                        refraction_ratio,
                    ) > rng.gen::<f64>()
                {
                    reflect(
                        unit_direction,
                        hit_record.normal,
                    )
                } else {
                    refract(
                        unit_direction,
                        hit_record.normal,
                        refraction_ratio,
                    )
                };

                Some(Scattered {
                    attenuation,
                    scattered: Ray {
                        origin: hit_record.point,
                        direction: direction,
                    },
                })
            }
        }
    }
}
