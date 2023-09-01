use glam::DVec3;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{
        //cylinder::Cylinder, rounded_box::RoundedBox,
        sphere::Sphere,
        Shapes,
    },
};
use std::io;

fn main() -> io::Result<()> {
    let mut world: Vec<Shapes> = vec![];

    let material_ground = Material::Lambertian {
        albedo: DVec3::new(0.8, 0.8, 0.0).into(),
    };
    let material_center = Material::Lambertian {
        albedo: DVec3::new(0., 0., 1.).into(),
    };

    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    // world.push(Shapes::Cylinder(Cylinder {
    //     start: DVec3::new(0.1, 0.1, -1.),
    //     end: DVec3::new(0.1, -0.1, -1.),
    //     radius: 10.,
    //     material: material_center,
    // }));

    let camera = Camera::init()
        .image_width(600)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(1., 1., 10.))
        .look_at(DVec3::NEG_Z)
        .vup(DVec3::Y)
        // .focus_dist(10.0)
        // .defocus_angle(0.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .build();

    camera.render_to_disk("cylinder", world)?;

    Ok(())
}
