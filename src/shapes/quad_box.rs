use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    textures::Texture,
};
use glam::DVec3;
use std::{f64::consts::PI, ops::Range};

use super::{quad::Quad, Shapes};

pub struct QuadBox {
    a: DVec3,
    b: DVec3,
    material: Material,
    objects: Vec<Shapes>,
}
impl QuadBox {
    pub fn new(
        a: DVec3,
        b: DVec3,
        material: Material,
    ) -> Self {
        let mut world = vec![];

        // Returns the 3D box (six sides) that contains the two opposite vertices a & b.

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = DVec3::new(
            a.x.min(b.x),
            a.y.min(b.y),
            a.z.min(b.z),
        );
        let max = DVec3::new(
            a.x.max(b.x),
            a.y.max(b.y),
            a.z.max(b.z),
        );

        let dx = DVec3::new(max.x - min.x, 0., 0.);
        let dy = DVec3::new(0., max.y - min.y, 0.);
        let dz = DVec3::new(0., 0., max.z - min.z);

        let front = Quad::new(
            DVec3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        );
        let right = Quad::new(
            DVec3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        );
        let back = Quad::new(
            DVec3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        );
        let left = Quad::new(
            DVec3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        );
        let top = Quad::new(
            DVec3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        );
        let bottom = Quad::new(
            DVec3::new(min.x, min.y, min.z),
            dx,
            dz,
            material.clone(),
        );
        world.push(Shapes::Quad(front));
        world.push(Shapes::Quad(right));
        world.push(Shapes::Quad(back));
        world.push(Shapes::Quad(left));
        world.push(Shapes::Quad(top));
        world.push(Shapes::Quad(bottom));

        Self {
            a,
            b,
            material,
            objects: world,
        }
    }
}

impl Hittable for QuadBox {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord> {
        self.objects.hit(ray, interval)
    }
}
