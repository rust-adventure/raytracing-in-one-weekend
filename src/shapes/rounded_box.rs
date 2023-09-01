use glam::{DVec2, DVec3, Vec3Swizzles};
use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

pub struct RoundedBox {
    pub center: DVec3,
    pub radius: f64,
    pub size: DVec3,
    pub material: Material,
}

// Rounded Box:     https://www.shadertoy.com/view/WlSXRW

impl Hittable for RoundedBox {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let ro = ray.origin;
        let rd = ray.direction;
        let dist_bound =
            DVec2::new(interval.start, interval.end);
        let size = self.size;
        let rad = self.radius;
        // normal unhandled

        // bounding box
        let m: DVec3 = rd.recip();
        let n: DVec3 = m * ro;
        let k: DVec3 = m.abs() * (size + rad);
        let t1: DVec3 = -n - k;
        let t2: DVec3 = -n + k;
        let tN: f64 = t1.x.max(t1.y).max(t1.z);
        let tF: f64 = t2.x.min(t2.y).min(t2.z);
        if tN > tF || tF < 0.0 {
            return None;
        }
        // let mut t: f64 = tN;
        let mut t: f64 =
            if tN >= dist_bound.x && tN <= dist_bound.y {
                tN
            } else {
                if interval.contains(&tF) {
                    tF
                } else {
                    // max_range
                    // dist_bound.y
                    1e100
                }
            };

        // convert to first octant
        let pos: DVec3 = ray.at(t);
        let s: DVec3 = pos.signum();
        let ros: DVec3 = ro * s;
        let rds: DVec3 = rd * s;
        let pos = pos * s;

        // faces
        let pos = pos - size;
        let pos = pos.xyz().max(pos.yzx());
        if pos.x.min(pos.y).min(pos.z) < 0.0 {
            if interval.contains(&t) {
                let p = ray.at(t);
                let normal = p.signum()
                    * (p.abs() - size.max(DVec3::ZERO))
                        .normalize();
                return Some(HitRecord::with_face_normal(
                    self.material.clone(),
                    p,
                    normal,
                    t,
                    ray,
                ));
            }
        }

        // some precomputation
        let oc: DVec3 = ros - size;
        let dd: DVec3 = rds * rds;
        let oo: DVec3 = oc * oc;
        let od: DVec3 = oc * rds;
        let ra2: f64 = rad * rad;

        t = interval.end;

        // corner
        {
            let b: f64 = od.x + od.y + od.z;
            let c: f64 = oo.x + oo.y + oo.z - ra2;
            let h: f64 = b * b - c;
            if h > 0.0 {
                t = -b - h.sqrt()
            };
        }

        // edge X
        {
            let a: f64 = dd.y + dd.z;
            let b: f64 = od.y + od.z;
            let c: f64 = oo.y + oo.z - ra2;
            let mut h: f64 = b * b - a * c;
            if h > 0.0 {
                h = (-b - h.sqrt()) / a;
                if h >= dist_bound.x
                    && h < t
                    && (ros.x + rds.x * h).abs() < size.x
                {
                    t = h
                };
            }
        }
        // edge Y
        {
            let a: f64 = dd.z + dd.x;
            let b: f64 = od.z + od.x;
            let c: f64 = oo.z + oo.x - ra2;
            let mut h: f64 = b * b - a * c;
            if h > 0.0 {
                h = (-b - h.sqrt()) / a;
                if h >= dist_bound.x
                    && h < t
                    && (ros.y + rds.y * h).abs() < size.y
                {
                    t = h
                };
            }
        }
        // edge Z
        {
            let a: f64 = dd.x + dd.y;
            let b: f64 = od.x + od.y;
            let c: f64 = oo.x + oo.y - ra2;
            let mut h: f64 = b * b - a * c;
            if h > 0.0 {
                h = (-b - h.sqrt()) / a;
                if h >= dist_bound.x
                    && h < t
                    && (ros.z + rds.z * h).abs() < size.z
                {
                    t = h
                };
            }
        }

        if interval.contains(&t) {
            let p = ray.at(t);
            let normal = p.signum()
                * (p.abs() - size)
                    .max(DVec3::splat(1e-16))
                    .normalize();
            return Some(HitRecord::with_face_normal(
                self.material.clone(),
                p,
                normal,
                t,
                ray,
            ));
        } else {
            return None;
        };
    }

    fn bounding_box(&self) -> crate::hittable::aabb::Aabb {
        todo!()
    }
}
// impl Hittable for RoundedBox {
//     fn hit(
//         &self,
//         ray: &Ray,
//         interval: Range<f64>,
//     ) -> Option<HitRecord> {
//         // let oc = ray.origin - self.center;
//         // let a = ray.direction.length_squared();
//         // let half_b = oc.dot(ray.direction);
//         // let c =
//         //     oc.length_squared() - self.radius * self.radius;

//         // let discriminant = half_b * half_b - a * c;
//         // if discriminant < 0. {
//         //     return None;
//         // }
//         // let sqrtd = discriminant.sqrt();

//         // // Find the nearest root that lies in the acceptable range.
//         // let mut root = (-half_b - sqrtd) / a;
//         // if !interval.contains(&root) {
//         //     root = (-half_b + sqrtd) / a;
//         //     if !interval.contains(&root) {
//         //         return None;
//         //     }
//         // }

//         // let t = root;
//         // let point = ray.at(t);
//         // let outward_normal =
//         //     (point - self.center) / self.

//         let tm: DVec2 =
//             intersect(ray.origin, ray.direction);

//         // convert position from world to box space
//         let bpos: DVec3 = ptransform(box_world_to_obj, pos);
//         // compute normal in box space
//         let bnor: DVec3 =
//             roundedbox_normal(bpos, box_size, box_radius);
//         // convert normal from box to world space
//         let nor: DVec3 = ntransform(box_obj_to_world, bnor);

//         let t = roundedbox_intersect(
//             ray.origin - self.center,
//             ray.direction - self.center,
//             self.size,
//             self.radius,
//         );
//         let point: DVec3 = ray.origin + t * ray.direction;
//         let outward_normal: DVec3 = roundedbox_normal(
//             point,
//             self.size,
//             self.radius,
//         );

//         let rec = HitRecord::with_face_normal(
//             self.material.clone(),
//             point,
//             outward_normal,
//             t,
//             ray,
//         );

//         Some(rec)
//     }
// }
// // intersect a ray with a rounded box
// // https://iquilezles.org/articles/intersectors
// fn roundedbox_intersect(
//     ro: DVec3,
//     rd: DVec3,
//     size: DVec3,
//     radius: f64,
// ) -> f64 {
//     // bounding box
//     let m: DVec3 = rd.recip();
//     let n: DVec3 = m * ro;
//     let k: DVec3 = m.abs() * (size + radius);
//     let t1: DVec3 = -n - k;
//     let t2: DVec3 = -n + k;
//     let tN: f64 = t1.x.max(t1.y).max(t1.z);
//     let tF: f64 = t2.x.min(t2.y).min(t2.z);
//     if tN > tF || tF < 0.0 {
//         return -1.0;
//     }
//     let mut t: f64 = tN;

//     // convert to first octant
//     let pos: DVec3 = ro + t * rd;
//     let s: DVec3 = pos.signum();
//     let ro = ro * s;
//     let rd = rd * s;
//     let mut pos = pos * s;

//     // faces
//     pos -= size;
//     pos = pos.xyz().max(pos.yzx());
//     if pos.x.min(pos.y).min(pos.z) < 0.0 {
//         return t;
//     }

//     // some precomputation
//     let oc: DVec3 = ro - size;
//     let dd: DVec3 = rd * rd;
//     let oo: DVec3 = oc * oc;
//     let od: DVec3 = oc * rd;
//     let ra2: f64 = radius * radius;

//     t = 1e20;

//     // corner
//     {
//         let b: f64 = od.x + od.y + od.z;
//         let c: f64 = oo.x + oo.y + oo.z - ra2;
//         let h: f64 = b * b - c;
//         if h > 0.0 {
//             t = -b - h.sqrt();
//         }
//     }

//     // edge X
//     {
//         let a: f64 = dd.y + dd.z;
//         let b: f64 = od.y + od.z;
//         let c: f64 = oo.y + oo.z - ra2;
//         let mut h: f64 = b * b - a * c;
//         if h > 0.0 {
//             h = (-b - h.sqrt()) / a;
//             if h > 0.0
//                 && h < t
//                 && (ro.x + rd.x * h).abs() < size.x
//             {
//                 t = h;
//             }
//         }
//     }
//     // edge Y
//     {
//         let a: f64 = dd.z + dd.x;
//         let b: f64 = od.z + od.x;
//         let c: f64 = oo.z + oo.x - ra2;
//         let mut h: f64 = b * b - a * c;
//         if h > 0.0 {
//             h = (-b - h.sqrt()) / a;
//             if h > 0.0
//                 && h < t
//                 && (ro.y + rd.y * h).abs() < size.y
//             {
//                 t = h;
//             }
//         }
//     }
//     // edge Z
//     {
//         let a: f64 = dd.x + dd.y;
//         let b: f64 = od.x + od.y;
//         let c: f64 = oo.x + oo.y - ra2;
//         let mut h: f64 = b * b - a * c;
//         if h > 0.0 {
//             h = (-b - h.sqrt()) / a;
//             if h > 0.0
//                 && h < t
//                 && (ro.z + rd.z * h).abs() < size.z
//             {
//                 t = h;
//             }
//         }
//     }

//     if t > 1e19 {
//         t = -1.0;
//     }

//     return t;
// }

// // normal of a rounded box
// fn roundedbox_normal(
//     pos: DVec3,
//     siz: DVec3,
//     rad: f64,
// ) -> DVec3 {
//     pos.signum() * pos.abs()
//         - siz.max(DVec3::ZERO).normalize()
// }

// fn intersect(ray: &Ray) -> DVec2 {
//     let mut res = DVec2::new(1e20, -1.0);

//     // rounded box

//     // convert ray from world to box space
//     let rdd: DVec3 =
//         ntransform(box_world_to_obj, ray.direction);
//     let roo: DVec3 =
//         ptransform(box_world_to_obj, ray.origin);
//     // intersect in box space
//     let t: f64 = roundedbox_intersect(
//         roo, rdd, box_size, box_radius,
//     );
//     if t > 0.0 && t < res.x {
//         res = DVec2::new(t, 2.0);
//     }

//     return res;
// }
