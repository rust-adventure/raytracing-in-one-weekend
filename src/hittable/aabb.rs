use crate::ray::Ray;
use glam::DVec3;
use std::ops::Range;

#[derive(Clone)]
pub struct Aabb {
    pub x: Range<f64>,
    pub y: Range<f64>,
    pub z: Range<f64>,
}

impl Aabb {
    pub fn axis(&self, n: i32) -> Range<f64> {
        match n {
            1 => self.y.clone(),
            2 => self.z.clone(),
            _ => self.x.clone(),
        }
    }
    pub fn hit(
        &self,
        ray: &Ray,
        mut ray_t: Range<f64>,
    ) -> bool {
        for a in [0, 1, 2] {
            let inv_d = ray.direction[a].recip();
            let orig = ray.origin[a];

            let mut t0 =
                (self.axis(a as i32).start - orig) * inv_d;
            let mut t1 =
                (self.axis(a as i32).end - orig) * inv_d;

            if inv_d < 0. {
                (t0, t1) = (t1, t0);
            }

            if t0 > ray_t.start {
                ray_t.start = t0;
            }
            if t1 < ray_t.end {
                ray_t.end = t1;
            }

            if ray_t.end <= ray_t.start {
                return false;
            }
        }
        return true;
    }
}

impl From<(DVec3, DVec3)> for Aabb {
    fn from((a, b): (DVec3, DVec3)) -> Self {
        Aabb {
            x: a.x.min(b.x)..a.x.max(b.x),
            y: a.y.min(b.y)..a.y.max(b.y),
            z: a.z.min(b.z)..a.z.max(b.z),
        }
    }
}

impl From<(Aabb, Aabb)> for Aabb {
    fn from((box0, box1): (Aabb, Aabb)) -> Self {
        Aabb {
            x: merge_range(box0.x, box1.x),
            y: merge_range(box0.y, box1.y),
            z: merge_range(box0.z, box1.z),
        }
    }
}

fn merge_range(a: Range<f64>, b: Range<f64>) -> Range<f64> {
    a.start.min(b.start)..a.end.max(b.end)
}
