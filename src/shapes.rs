use glam::DVec3;

use crate::{hittable::Hittable, ray::Ray};

pub mod quad;
pub mod quad_box;
pub mod sphere;
// are other shapes useful?
// possible SDF definitions?
// pub mod a_box;
// pub mod cylinder;
// pub mod rounded_box;

pub enum Shapes {
    Sphere(sphere::Sphere),
    Quad(quad::Quad),
    QuadBox(quad_box::QuadBox),
    Translate {
        offset: DVec3,
        object: Box<Shapes>,
    },
    RotateY {
        sin_theta: f64,
        cos_theta: f64,
        object: Box<Shapes>,
    },
    // RoundedBox(rounded_box::RoundedBox),
    // Box(a_box::Box),
    // Cylinder(cylinder::Cylinder),
}
impl Shapes {
    pub fn new_rotate_y(
        angle: f64,
        object: Shapes,
    ) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        Self::RotateY {
            sin_theta,
            cos_theta,
            object: Box::new(object),
        }
    }
}

impl Hittable for Shapes {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        interval: std::ops::Range<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        match self {
            Shapes::Sphere(object) => {
                object.hit(ray, interval)
            }
            Shapes::Quad(object) => {
                object.hit(ray, interval)
            }
            Shapes::QuadBox(object) => {
                object.hit(ray, interval)
            }
            Shapes::Translate { offset, object } => {
                // Move the ray backwards by the offset
                let offset_ray = Ray {
                    origin: ray.origin - *offset,
                    direction: ray.direction,
                    time: ray.time,
                };
                // Determine where (if any) an intersection occurs along the offset ray
                let Some(mut hit_record) =
                    object.hit(&offset_ray, interval)
                else {
                    return None;
                };
                // Move the intersection point forwards by the offset
                hit_record.point += *offset;
                Some(hit_record)
            }
            Shapes::RotateY {
                sin_theta,
                cos_theta,
                object,
            } => {
                // Change the ray from world space to object space
                let mut origin = ray.origin.clone();
                let mut direction = ray.direction.clone();

                origin.x = cos_theta * ray.origin.x
                    - sin_theta * ray.origin.z;
                origin.z = sin_theta * ray.origin.x
                    + cos_theta * ray.origin.z;

                direction.x = cos_theta * ray.direction.x
                    - sin_theta * ray.direction.z;
                direction.z = sin_theta * ray.direction.x
                    + cos_theta * ray.direction.z;

                let rotated_r = Ray {
                    origin,
                    direction,
                    time: ray.time,
                };

                // Determine where (if any) an intersection occurs in object space
                let Some(mut hit_record) =
                    object.hit(&rotated_r, interval)
                else {
                    return None;
                };

                // Change the intersection point from object space to world space
                let mut p = hit_record.point;
                p.x = cos_theta * hit_record.point.x
                    + sin_theta * hit_record.point.z;
                p.z = -sin_theta * hit_record.point.x
                    + cos_theta * hit_record.point.z;

                // Change the normal from object space to world space
                let mut normal = hit_record.normal;
                normal.x = cos_theta * hit_record.normal.x
                    + sin_theta * hit_record.normal.z;
                normal.z = -sin_theta * hit_record.normal.x
                    + cos_theta * hit_record.normal.z;

                hit_record.point = p;
                hit_record.normal = normal;

                Some(hit_record)
            }
        }
    }
}
