use glam::DVec3;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{
        a_box, cylinder::Cylinder, sphere::Sphere, Shapes,
    },
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

    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Shapes::Box(a_box::Box {
        center: DVec3::new(0.0, 0.0, -1.0),
        size: DVec3::splat(0.2),
        material: material_center,
    }));
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.push(Shapes::Cylinder(Cylinder {
        start: DVec3::splat(-1.),
        end: DVec3::splat(-2.),
        radius: 0.5,
        material: material_right,
    }));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(-3., 4., 2.))
        .look_at(DVec3::NEG_Z)
        .vup(DVec3::Y)
        // .focus_dist(10.0)
        // .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(90.)
        .build();

    camera.render_to_disk("all-shapes", &*world)?;

    Ok(())
}
