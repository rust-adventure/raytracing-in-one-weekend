use glam::DVec3;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;

use std::{fs, io, ops::Range};
fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = HittableList { objects: vec![] };

    let ground_material = Material::Lambertian {
        albedo: DVec3::new(0.5, 0.5, 0.5),
    };
    world.add(Sphere {
        center: DVec3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    });
    for (a, b) in
        (-11..11).cartesian_product(-11..11).into_iter()
    {
        let choose_mat = rng.gen::<f64>();
        let center = DVec3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );

        if (center - DVec3::new(4., 0.2, 0.)).length() > 0.9
        {
            if choose_mat < 0.8 {
                // diffuse
                let albedo = DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                ) * DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                );
                let material =
                    Material::Lambertian { albedo: albedo };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let albedo = DVec3::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );
                let fuzz = rng.gen_range(0f64..0.5);
                let material =
                    Material::Metal { albedo, fuzz };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    let material1 = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    world.add(Sphere {
        center: DVec3::new(0., 1., 0.),
        radius: 1.0,
        material: material1,
    });

    let material2 = Material::Lambertian {
        albedo: DVec3::new(0.4, 0.2, 0.1),
    };
    world.add(Sphere {
        center: DVec3::new(-4., 1., 0.),
        radius: 1.0,
        material: material2,
    });

    let material3 = Material::Metal {
        albedo: DVec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Sphere {
        center: DVec3::new(4., 1., 0.),
        radius: 1.0,
        material: material3,
    });

    let camera = Camera::new(CameraNew {
        image_width: 1200,
        aspect_ratio: 16.0 / 9.0,
        look_from: Some(DVec3::new(13., 2., 3.)),
        look_at: Some(DVec3::ZERO),
        vup: Some(DVec3::Y),
        focus_dist: Some(10.0),
        defocus_angle: Some(0.0),
    });
    camera.render_to_disk(world)?;

    Ok(())
}

/// Hidden docs are calculated fields
struct Camera {
    /// Rendered image width in pixel count
    image_width: u32,
    #[doc(hidden)]
    image_height: u32,
    #[doc(hidden)]
    max_value: u8,
    /// Ratio of image width over height
    aspect_ratio: f64,
    #[doc(hidden)]
    center: DVec3,
    #[doc(hidden)]
    pixel_delta_u: DVec3,
    #[doc(hidden)]
    pixel_delta_v: DVec3,
    // viewport_upper_left: DVec3,
    #[doc(hidden)]
    pixel00_loc: DVec3,
    /// Count of random samples for each pixel
    samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Vertical view angle (field of view)
    vfov: f64,
    /// Point camera is looking from
    lookfrom: DVec3,
    /// Point camera is looking at
    lookat: DVec3,
    /// Camera-relative "up" direction
    vup: DVec3,

    /// basis vectors
    #[doc(hidden)]
    u: DVec3,
    #[doc(hidden)]
    v: DVec3,
    #[doc(hidden)]
    w: DVec3,

    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    focus_dist: f64,
    /// Defocus disk horizontal radius
    defocus_disk_u: DVec3,
    /// Defocus disk vertical radius
    defocus_disk_v: DVec3,
}
struct CameraNew {
    image_width: u32,
    aspect_ratio: f64,
    look_from: Option<DVec3>,
    look_at: Option<DVec3>,
    vup: Option<DVec3>,
    focus_dist: Option<f64>,
    defocus_angle: Option<f64>,
}
impl Camera {
    fn new(config: CameraNew) -> Self {
        let lookfrom =
            config.look_from.unwrap_or(DVec3::NEG_Z);
        let lookat = config.look_at.unwrap_or(DVec3::ZERO);
        let vup = config.vup.unwrap_or(DVec3::Y);
        let focus_dist = config.focus_dist.unwrap_or(10.);
        let defocus_angle =
            config.defocus_angle.unwrap_or(0.);

        let max_value: u8 = 255;
        let image_height: u32 = (config.image_width as f64
            / config.aspect_ratio)
            as u32;

        let vfov: f64 = 20.0;
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * focus_dist;
        let viewport_width: f64 = viewport_height
            * (config.image_width as f64
                / image_height as f64);

        let center: DVec3 = lookfrom;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        // ## Calculate the vectors across the horizontal and down the vertical viewport edges.
        // Vector across viewport horizontal edge
        let viewport_u = viewport_width * u;
        // Vector down viewport vertical edge
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: DVec3 =
            viewport_u / config.image_width as f64;
        let pixel_delta_v: DVec3 =
            viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: DVec3 = center
            - focus_dist * w
            - viewport_u / 2.
            - viewport_v / 2.;
        let pixel00_loc: DVec3 = viewport_upper_left
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        //   no tan: 0.296705972839036
        // with tan: 0.29746145598814155
        let defocus_radius = focus_dist
            * (defocus_angle / 2.).to_radians().tan();

        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width: config.image_width,
            image_height,
            max_value,
            aspect_ratio: config.aspect_ratio,
            center,
            pixel_delta_u,
            pixel_delta_v,
            // viewport_upper_left,
            pixel00_loc,
            samples_per_pixel: 500,
            max_depth: 50,
            vfov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.

        let pixel_center = self.pixel00_loc
            + (i as f64 * self.pixel_delta_u)
            + (j as f64 * self.pixel_delta_v);
        let pixel_sample =
            pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray {
            origin: self.center,
            direction: ray_direction,
        }
    }
    fn defocus_disk_sample(&self) -> DVec3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center
            + (p.x * self.defocus_disk_u)
            + (p.y * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> DVec3 {
        let mut rng = rand::thread_rng();
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        (px * self.pixel_delta_u)
            + (py * self.pixel_delta_v)
    }
    fn render_to_disk<T>(&self, world: T) -> io::Result<()>
    where
        T: Hittable + std::marker::Sync,
    {
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .progress_count(
                self.image_height as u64
                    * self.image_width as u64,
            )
            .map(|(y, x)| {
                let scale_factor =
                    (self.samples_per_pixel as f64).recip();

                let multisampled_pixel_color = (0..self
                    .samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        self.get_ray(x as i32, y as i32)
                            .color(
                                self.max_depth as i32,
                                &world,
                            )
                    })
                    .sum::<DVec3>()
                    * scale_factor;

                // * 256.
                let color = DVec3 {
                    x: linear_to_gamma(
                        multisampled_pixel_color.x,
                    ),
                    y: linear_to_gamma(
                        multisampled_pixel_color.y,
                    ),
                    z: linear_to_gamma(
                        multisampled_pixel_color.z,
                    ),
                }
                .clamp(
                    DVec3::splat(0.),
                    DVec3::splat(0.999),
                ) * 256.;
                format!(
                    "{} {} {}",
                    color.x, color.y, color.z
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(
            "output.ppm",
            format!(
                "P3
{} {}
{}
{pixels}
",
                self.image_width,
                self.image_height,
                self.max_value
            ),
        )
    }
}

fn linear_to_gamma(scalar: f64) -> f64 {
    scalar.sqrt()
}

struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color<T>(&self, depth: i32, world: &T) -> DVec3
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

trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
    ) -> Option<HitRecord>;
}

#[non_exhaustive]
#[derive(Clone)]
enum Material {
    Lambertian { albedo: DVec3 },
    Metal { albedo: DVec3, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}
struct Scattered {
    attenuation: DVec3,
    scattered: Ray,
}
impl Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: HitRecord,
    ) -> Option<Scattered> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record
                    .normal
                    + random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.abs_diff_eq(
                    DVec3::new(0., 0., 0.),
                    1e-8,
                ) {
                    scatter_direction = hit_record.normal;
                }

                let scattered = Ray {
                    origin: hit_record.point,
                    direction: scatter_direction,
                };

                Some(Scattered {
                    attenuation: *albedo,
                    scattered,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected: DVec3 = reflect(
                    r_in.direction.normalize(),
                    hit_record.normal,
                );
                let scattered = Ray {
                    origin: hit_record.point,
                    direction: reflected
                        + *fuzz * random_unit_vector(),
                };
                // absorb any scatter that is below the surface
                if scattered
                    .direction
                    .dot(hit_record.normal)
                    > 0.
                {
                    Some(Scattered {
                        attenuation: *albedo,
                        scattered,
                    })
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let mut rng = rand::thread_rng();

                let attenuation = DVec3::splat(1.0);
                let refraction_ratio: f64 =
                    if hit_record.front_face {
                        index_of_refraction.recip()
                    } else {
                        *index_of_refraction
                    };

                let unit_direction =
                    r_in.direction.normalize();

                let cos_theta = (-unit_direction
                    .dot(hit_record.normal))
                .min(1.0);
                let sin_theta =
                    (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract =
                    refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract
                    || reflectance(
                        cos_theta,
                        refraction_ratio,
                    ) > rng.gen::<f64>()
                {
                    reflect(
                        unit_direction,
                        hit_record.normal,
                    )
                } else {
                    refract(
                        unit_direction,
                        hit_record.normal,
                        refraction_ratio,
                    )
                };

                Some(Scattered {
                    attenuation,
                    scattered: Ray {
                        origin: hit_record.point,
                        direction: direction,
                    },
                })
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
struct HitRecord {
    point: DVec3,
    normal: DVec3,
    t: f64,
    front_face: bool,
    material: Material,
}
impl HitRecord {
    fn with_face_normal(
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

struct Sphere {
    center: DVec3,
    radius: f64,
    material: Material,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        interval: Range<f64>,
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
            self.material.clone(),
            point,
            outward_normal,
            t,
            ray,
        );

        Some(rec)
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync>>,
}
impl HittableList {
    fn clear(&mut self) {
        self.objects = vec![]
    }

    fn add<T>(&mut self, object: T)
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

fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );

        if vec.length_squared() < 1. {
            break vec;
        }
    }
}

fn random_unit_vector() -> DVec3 {
    return random_in_unit_sphere().normalize();
}

fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0
    // In the same hemisphere as the normal
    {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    return v - 2. * v.dot(n) * n;
}

fn refract(
    uv: DVec3,
    n: DVec3,
    etai_over_etat: f64,
) -> DVec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: DVec3 =
        -((1.0 - r_out_perp.length_squared()).abs()).sqrt()
            * n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}

fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            0.,
        );

        if v.length_squared() < 1. {
            break v;
        }
    }
}
