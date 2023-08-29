use crate::{hittable::Hittable, ray::Ray};
use glam::DVec3;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs, io};

mod builder;
use builder::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Camera {
    /// Rendered image width in pixel count
    image_width: u32,
    /// calculated image_height
    image_height: u32,
    /// max color value (0-255 by default)
    max_value: u8,
    /// Ratio of image width over height
    aspect_ratio: f64,
    /// center of camera
    center: DVec3,
    /// how far to move in the u direction to get to the next pixel
    pixel_delta_u: DVec3,
    /// how far to move in the v direction to get to the next pixel
    pixel_delta_v: DVec3,
    // viewport_upper_left: DVec3,
    /// the location of the pixel at 0,0
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

    /// basis vector u
    u: DVec3,
    /// basis vector v
    v: DVec3,
    /// basis vector w
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

impl Default for Camera {
    fn default() -> Self {
        Self::init().build()
    }
}
impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
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
                            .color(self.max_depth, &world)
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
