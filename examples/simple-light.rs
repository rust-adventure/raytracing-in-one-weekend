use glam::DVec3;
use noise::Perlin;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{quad::Quad, sphere::Sphere, Shapes},
    textures::Texture,
};
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let mut world = vec![];

    // let perlin = Perlin::new(1);
    // let pertext = Texture::PerlinNoise(perlin, 2.);
    let perlin = Perlin::new(1);
    let pertext = Texture::Turbulence(perlin);

    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: pertext.clone(),
        },
    )));
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(0., 2., 0.),
        2.,
        Material::Lambertian { albedo: pertext },
    )));

    let difflight = Material::DiffuseLight(
        DVec3::new(4., 4., 4.).into(),
    );

    // Quads
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(3., 1., -2.),
        DVec3::new(2., 0., 0.),
        DVec3::new(0., 2., 0.),
        difflight,
    )));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16. / 9.)
        .look_from(DVec3::new(23., 3., 6.))
        .look_at(DVec3::new(0., 2., 0.))
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.)
        .background(DVec3::ZERO)
        .build();

    let filename = Path::new(file!())
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap()
        .trim_end_matches(".rs");
    camera.render_to_disk(filename, world)?;

    Ok(())
}
