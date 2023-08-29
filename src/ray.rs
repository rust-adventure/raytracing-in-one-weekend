use glam::DVec3;

use crate::{hittable::Hittable, material::Scattered};

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    pub fn color<T>(&self, depth: u32, world: &T) -> DVec3
    where
        T: Hittable + std::marker::Sync,
    {
        // depth == 0 means we're done
        if depth <= 0 {
            return DVec3::ZERO;
        }
        // if we hit something
        if let Some(rec) =
            world.hit(&self, (0.001)..f64::INFINITY)
        {
            // scatter rays on the material we hit IF
            // the material wants to scatter them.
            // the material is allowed to absorb rays by
            // returning None
            let Some(Scattered {
                attenuation,
                scattered,
            }) = rec.material.scatter(self, &rec)
            else {
                return DVec3::ZERO;
            };

            // recurse to follow more bounces
            return attenuation
                * scattered.color(depth - 1, world);
        }

        // this is sky because we missed everything
        let a = 0.5 * (self.direction.normalize().y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0)
            + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
