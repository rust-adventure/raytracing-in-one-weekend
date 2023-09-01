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
    let left_red = Material::Lambertian {
        albedo: DVec3::new(1.0, 0.2, 0.2).into(),
    };
    let back_green = Material::Lambertian {
        albedo: DVec3::new(0.2, 1.0, 0.2).into(),
    };
    let right_blue = Material::Lambertian {
        albedo: DVec3::new(0.2, 0.2, 1.0).into(),
    };
    let upper_orange = Material::Lambertian {
        albedo: DVec3::new(1.0, 0.5, 0.0).into(),
    };
    let lower_teal = Material::Lambertian {
        albedo: DVec3::new(0.2, 0.8, 0.8).into(),
    };

    // Quads
    world.push(Quad::new(
        DVec3::new(-3., -2., 5.),
        DVec3::new(0., 0., -4.),
        DVec3::new(0., 4., 0.),
        left_red,
    ));
    world.push(Quad::new(
        DVec3::new(-2., -2., 0.),
        DVec3::new(4., 0., 0.),
        DVec3::new(0., 4., 0.),
        back_green,
    ));
    world.push(Quad::new(
        DVec3::new(3., -2., 1.),
        DVec3::new(0., 0., 4.),
        DVec3::new(0., 4., 0.),
        right_blue,
    ));
    world.push(Quad::new(
        DVec3::new(-2., 3., 1.),
        DVec3::new(4., 0., 0.),
        DVec3::new(0., 0., 4.),
        upper_orange,
    ));
    world.push(Quad::new(
        DVec3::new(-2., -3., 5.),
        DVec3::new(4., 0., 0.),
        DVec3::new(0., 0., -4.),
        lower_teal,
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(1.)
        .look_from(DVec3::new(0., 0., 9.))
        .look_at(DVec3::ZERO)
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(80.)
        // .background(DVec3::new(0.70, 0.80, 1.00))
        .build();

    let filename = Path::new(file!())
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap()
        .trim_end_matches(".rs");
    camera.render_to_disk(filename, world)?;

    Ok(())
}
