use std::ops::Range;

use glam::{DVec3, Vec3Swizzles};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

pub struct Box {
    pub center: DVec3,
    pub size: DVec3,
    pub material: Material,
}

impl Hittable for Box {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let m: DVec3 = ray.direction.signum()
            / ray.direction.abs().max(DVec3::splat(1e-8));
        let n: DVec3 = m * ray.origin;
        let k: DVec3 = m.abs() * self.size;

        let t1: DVec3 = -n - k;
        let t2: DVec3 = -n + k;

        let tN: f64 = t1.x.max(t1.y).max(t1.z);
        let tF: f64 = t2.x.min(t2.y).min(t2.z);

        if tN > tF || tF <= 0. {
            return None;
        } else {
            if interval.contains(&tN) {
                let normal = -ray.direction.signum()
                    * step(t1.yzx(), t1.xyz())
                    * step(t1.zxy(), t1.xyz());
                // return tN;
                return Some(HitRecord::with_face_normal(
                    self.material.clone(),
                    ray.at(tN),
                    normal,
                    tN,
                    ray,
                ));
            } else if interval.contains(&tF) {
                let normal = -ray.direction.signum()
                    * step(t1.yzx(), t1.xyz())
                    * step(t1.zxy(), t1.xyz());
                // return tF;
                return Some(HitRecord::with_face_normal(
                    self.material.clone(),
                    ray.at(tF),
                    normal,
                    tF,
                    ray,
                ));
            } else {
                return None;
            }
        }
        // }

        // let rec = HitRecord::with_face_normal(
        //     self.material.clone(),
        //     point,
        //     outward_normal,
        //     t,
        //     ray,
        // );

        // Some(rec)
    }
}

fn step(edge: DVec3, x: DVec3) -> DVec3 {
    let bvec = x.cmpge(edge);
    DVec3::new(
        if bvec.x { 1.0 } else { 0.0 },
        if bvec.y { 1.0 } else { 0.0 },
        if bvec.z { 1.0 } else { 0.0 },
    )
}
