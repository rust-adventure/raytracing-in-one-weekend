use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use glam::{DVec3, Vec3Swizzles};
use std::ops::Range;

pub struct Cylinder {
    pub start: DVec3,
    pub end: DVec3,
    pub radius: f64,
    pub material: Material,
}
// return Some(HitRecord::with_face_normal(
//     self.material.clone(),
//     ray.at(tF),
//     normal,
//     tF,
//     ray,
// ));
impl Hittable for Cylinder {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let pa = self.start;
        let pb = self.end;
        let ca: DVec3 = pb - pa;
        let oc: DVec3 = ray.origin - pa;

        let caca: f64 = ca.dot(ca);
        let card: f64 = ca.dot(ray.direction);
        let caoc: f64 = ca.dot(oc);

        let a: f64 = caca - card * card;
        let b: f64 =
            caca * oc.dot(ray.direction) - caoc * card;
        let c: f64 = caca * oc.dot(oc)
            - caoc * caoc
            - self.radius * self.radius * caca;
        let mut h: f64 = b * b - a * c;

        if h < 0. {
            return None;
        }

        h = h.sqrt();
        let mut d: f64 = (-b - h) / a;

        let y: f64 = caoc + d * card;
        if y > 0. && y < caca && interval.contains(&d) {
            let normal = (oc + d * ray.direction
                - ca * y / caca)
                / self.radius;
            return Some(HitRecord::with_face_normal(
                self.material.clone(),
                ray.at(d),
                // oc + d * ray.direction,
                normal,
                d,
                ray,
            ));
        }

        d = ((if y < 0. { 0. } else { caca }) - caoc)
            / card;

        if (b + a * d).abs() < h && interval.contains(&d) {
            let normal =
                (ca * y.signum() / caca).normalize();
            return Some(HitRecord::with_face_normal(
                self.material.clone(),
                ray.at(d),
                normal,
                d,
                ray,
            ));
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> crate::hittable::aabb::Aabb {
        todo!()
    }
}

// float iCylinder( in vec3 ro, in vec3 rd, in vec2 distBound, inout vec3 normal,
//     in vec3 pa, in vec3 pb, float ra ) {

// }
