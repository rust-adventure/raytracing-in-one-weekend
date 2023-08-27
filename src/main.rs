use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io, ops::Range};

const IMAGE_WIDTH: u32 = 400;
const MAX_VALUE: u8 = 255;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 =
    (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT
    * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: DVec3 = DVec3::ZERO;

// Calculate the vectors across the horizontal and down the vertical viewport edges.
const VIEWPORT_U: DVec3 =
    DVec3::new(VIEWPORT_WIDTH, 0., 0.);
const VIEWPORT_V: DVec3 =
    DVec3::new(0., -VIEWPORT_HEIGHT, 0.);

fn main() -> io::Result<()> {
    let mut world = HittableList { objects: vec![] };

    world.add(Sphere {
        center: DVec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: DVec3::new(0., -100.5, -1.),
        radius: 100.,
    });

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: DVec3 =
        VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 =
        VIEWPORT_V / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: DVec3 = CAMERA_CENTER
        - DVec3::new(0., 0., FOCAL_LENGTH)
        - VIEWPORT_U / 2.
        - VIEWPORT_V / 2.;
    let pixel00_loc: DVec3 = viewport_upper_left
        + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(
            IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64,
        )
        .map(|(y, x)| {
            let pixel_center = pixel00_loc
                + (x as f64 * pixel_delta_u)
                + (y as f64 * pixel_delta_v);
            let ray_direction =
                pixel_center - CAMERA_CENTER;
            let ray = Ray {
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };

            let pixel_color = ray.color(&world) * 255.0;

            format!(
                "{} {} {}",
                pixel_color.x, pixel_color.y, pixel_color.z
            )
        })
        .join("\n");
    fs::write(
        "output.ppm",
        format!(
            "P3
{IMAGE_WIDTH} {IMAGE_HEIGHT}
{MAX_VALUE}
{pixels}
"
        ),
    )?;
    Ok(())
}

struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color<T>(&self, world: &T) -> DVec3
    where
        T: Hittable,
    {
        if let Some(rec) =
            world.hit(&self, (0.)..f64::INFINITY)
        {
            return 0.5
                * (rec.normal + DVec3::new(1., 1., 1.));
        }

        let unit_direction: DVec3 =
            self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0)
            + a * DVec3::new(0.5, 0.7, 1.0);
    }
}

// fn hit_sphere(
//     center: &DVec3,
//     radius: f64,
//     ray: &Ray,
// ) -> f64 {
//     let oc: DVec3 = ray.origin - *center;
//     let a = ray.direction.length_squared();
//     let half_b = oc.dot(ray.direction);
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;

//     if discriminant < 0. {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / a
//     }
// }

trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
        // ray_tmin: f64,
        // ray_tmax: f64,
        // record: HitRecord,
    ) -> Option<HitRecord>;
}

struct HitRecord {
    point: DVec3,
    normal: DVec3,
    t: f64,
    front_face: bool,
}
impl HitRecord {
    fn with_face_normal(
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

struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
        // ray_tmin: f64,
        // ray_tmax: f64,
        // record: HitRecord,
    ) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c =
            oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !interval.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal =
            (point - self.center) / self.radius;

        let rec = HitRecord::with_face_normal(
            point,
            outward_normal,
            t,
            ray,
        );

        Some(rec)
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    fn clear(&mut self) {
        self.objects = vec![]
    }

    fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
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
        // ray_tmin: f64,
        // ray_tmax: f64,
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
