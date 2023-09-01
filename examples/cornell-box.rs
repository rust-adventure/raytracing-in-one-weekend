use glam::DVec3;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{quad::Quad, sphere::Sphere},
    textures::Texture,
};
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let mut world = vec![];

    // Materials
    let red = Material::Lambertian {
        albedo: DVec3::new(0.65, 0.05, 0.05).into(),
    };
    let white = Material::Lambertian {
        albedo: DVec3::new(0.73, 0.73, 0.73).into(),
    };
    let green = Material::Lambertian {
        albedo: DVec3::new(0.12, 0.45, 0.15).into(),
    };
    let light = Material::DiffuseLight(
        DVec3::new(15., 15., 15.).into(),
    );

    world.push(Quad::new(
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 555., 0.),
        DVec3::new(0., 0., 555.),
        green,
    ));
    world.push(Quad::new(
        DVec3::new(0., 0., 0.),
        DVec3::new(0., 555., 0.),
        DVec3::new(0., 0., 555.),
        red,
    ));
    world.push(Quad::new(
        DVec3::new(343., 554., 332.),
        DVec3::new(-130., 0., 0.),
        DVec3::new(0., 0., -105.),
        light,
    ));
    world.push(Quad::new(
        DVec3::new(0., 0., 0.),
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 0., 555.),
        white.clone(),
    ));
    world.push(Quad::new(
        DVec3::new(555., 555., 555.),
        DVec3::new(-555., 0., 0.),
        DVec3::new(0., 0., -555.),
        white.clone(),
    ));
    world.push(Quad::new(
        DVec3::new(0., 0., 555.),
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 555., 0.),
        white,
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(1.)
        .look_from(DVec3::new(278., 278., -800.))
        .look_at(DVec3::new(278., 278., 0.))
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(40.)
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
