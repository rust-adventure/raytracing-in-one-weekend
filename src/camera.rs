use std::{fs, io};

use glam::DVec3;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{hittable::Hittable, ray::Ray};

/// Hidden docs are calculated fields
pub struct Camera {
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
pub struct CameraNew {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub look_from: Option<DVec3>,
    pub look_at: Option<DVec3>,
    pub vup: Option<DVec3>,
    pub focus_dist: Option<f64>,
    pub defocus_angle: Option<f64>,
}
impl Camera {
    pub fn new(config: CameraNew) -> Self {
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
    pub fn render_to_disk<T>(
        &self,
        world: T,
    ) -> io::Result<()>
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

/// unit disk is used to power the base of the focus
/// cone. We shoot rays from randomized locations on the
/// unit disk instead of directly from the center to power blur.
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
