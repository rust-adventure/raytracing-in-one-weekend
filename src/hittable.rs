use std::ops::Range;

use glam::DVec3;

use crate::{material::Material, ray::Ray};

pub trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    t: f64,
    pub front_face: bool,
    pub material: Material,
}
impl HitRecord {
    pub fn with_face_normal(
        material: Material,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        ray: &Ray,
    ) -> Self {
        let (front_face, normal) =
            HitRecord::calc_face_normal(
                ray,
                &outward_normal,
            );
        HitRecord {
            material,
            point,
            normal,
            t,
            front_face,
        }
    }
    fn calc_face_normal(
        ray: &Ray,
        outward_normal: &DVec3,
    ) -> (bool, DVec3) {
        // TODO: Why is outward_normal.is_normalized() false
        // for some normals for which these two values are exactly the same:
        // dbg!(
        //     outward_normal,
        //     outward_normal.normalize()
        // );
        // debug_assert!(
        //     !outward_normal.is_normalized(),
        //     "outward_normal must be normalized"
        // );

        let front_face =
            ray.direction.dot(*outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }
    // Unused
    fn set_face_normal(
        &mut self,
        ray: &Ray,
        outward_normal: &DVec3,
    ) {
        let (front_face, normal) =
            HitRecord::calc_face_normal(
                ray,
                outward_normal,
            );

        self.front_face = front_face;
        self.normal = normal;
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
}
impl HittableList {
    fn clear(&mut self) {
        self.objects = vec![]
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static + Sync,
    {
        // was push_back
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        let (_closest, hit_record) = self
            .objects
            .iter()
            .fold((interval.end, None), |acc, item| {
                if let Some(temp_rec) = item.hit(
                    ray,
                    interval.start..acc.0,
                    // acc.0,
                ) {
                    (temp_rec.t, Some(temp_rec))
                } else {
                    acc
                }
            });

        hit_record
    }
}
