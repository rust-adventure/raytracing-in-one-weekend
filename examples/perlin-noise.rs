use glam::DVec3;
use noise::{NoiseFn, Perlin, Seedable};
use raytracer::{
    camera::Camera, material::Material,
    shapes::sphere::Sphere, textures::Texture,
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = vec![];

    // let perlin = Perlin::new(1);
    // let noise_texture = Texture::PerlinNoise(perlin, 10.);

    let perlin = Perlin::new(1);
    let noise_texture = Texture::Turbulence(perlin);

    world.push(Sphere::new(
        DVec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: noise_texture.clone(),
        },
    ));
    world.push(Sphere::new(
        DVec3::new(0., 2., 0.),
        2.,
        Material::Lambertian {
            albedo: noise_texture,
        },
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(12., 2., 3.))
        .look_at(DVec3::ZERO)
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .build();

    camera.render_to_disk("perlin-noise", world)?;

    Ok(())
}
