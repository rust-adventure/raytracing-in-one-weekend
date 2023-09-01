use crate::hittable::Hittable;

pub mod sphere;
// are other shapes useful?
// possible SDF definitions?
// pub mod a_box;
// pub mod cylinder;
// pub mod rounded_box;

pub enum Shapes {
    Sphere(sphere::Sphere),
    // RoundedBox(rounded_box::RoundedBox),
    // Box(a_box::Box),
    // Cylinder(cylinder::Cylinder),
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
            } // Shapes::RoundedBox(object) => {
              //     object.hit(ray, interval)
              // }
              // Shapes::Box(object) => {
              //     object.hit(ray, interval)
              // }
              // Shapes::Cylinder(object) => {
              //     object.hit(ray, interval)
              // }
        }
    }
}
