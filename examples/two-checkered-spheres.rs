use glam::DVec3;
use itertools::Itertools;
use rand::prelude::*;
use raytracer::{
    camera::Camera, material::Material,
    shapes::sphere::Sphere, textures::Texture,
};
use std::io;

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = vec![];
    // (0.32, color(.2, .3, .1), color(.9, .9, .9))
    let checker = Texture::Checkered {
        even: DVec3::new(0.2, 0.3, 0.1),
        odd: DVec3::splat(0.9),
        scale: 0.32,
    };

    world.push(Sphere::new(
        DVec3::new(0., -10., 0.),
        10.,
        Material::Lambertian {
            albedo: checker.clone(),
        },
    ));
    world.push(Sphere::new(
        DVec3::new(0., 10., 0.),
        10.,
        Material::Lambertian { albedo: checker },
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(13., 2., 3.))
        .look_at(DVec3::ZERO)
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .build();

    camera
        .render_to_disk("two-checkered-spheres", world)?;

    Ok(())
}
