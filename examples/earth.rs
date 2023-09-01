use glam::DVec3;
use raytracer::{
    camera::Camera, material::Material,
    shapes::sphere::Sphere, textures::Texture,
};
use std::io;

fn main() -> io::Result<()> {
    let earth_texture =
        Texture::load_image("assets/earthmap.jpg")?;

    let mut world = vec![];

    world.push(Sphere::new(
        DVec3::new(0., 0., 0.),
        2.,
        Material::Lambertian {
            albedo: earth_texture,
        },
    ));

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(0., 0., 12.))
        .look_at(DVec3::ZERO)
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .build();

    camera.render_to_disk("earth", world)?;

    Ok(())
}
