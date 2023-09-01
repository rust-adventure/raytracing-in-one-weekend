use glam::DVec3;
use raytracer::{
    camera::Camera, material::Material,
    shapes::sphere::Sphere,
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = vec![];

    let material_ground = Material::Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: DVec3::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    let material_right = Material::Metal {
        albedo: DVec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    world.push(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(
        DVec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.push(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.push(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ));
    world.push(Sphere::new(
        DVec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(-2., 2., 1.))
        .look_at(DVec3::NEG_Z)
        .vup(DVec3::Y)
        // .focus_dist(10.0)
        // .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .build();

    camera
        .render_to_disk("all-materials-spheres", &*world)?;

    Ok(())
}
