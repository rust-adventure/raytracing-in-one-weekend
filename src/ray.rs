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
    pub fn color<T>(&self, depth: i32, world: &T) -> DVec3
    where
        T: Hittable + std::marker::Sync,
    {
        if depth <= 0 {
            return DVec3::new(0., 0., 0.);
        }
        if let Some(rec) =
            world.hit(&self, (0.001)..f64::INFINITY)
        {
            if let Some(Scattered {
                attenuation,
                scattered,
            }) = rec.material.scatter(self, rec.clone())
            {
                return attenuation
                    * scattered.color(depth - 1, world);
            }
            return DVec3::new(0., 0., 0.);
        }

        let unit_direction: DVec3 =
            self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0)
            + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
