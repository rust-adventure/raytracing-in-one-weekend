use glam::DVec3;
use itertools::Itertools;
use rand::prelude::*;
use raytracer::{
    camera::Camera, material::Material,
    shapes::sphere::Sphere,
};
use std::io;

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = vec![];

    world.push(Sphere::new(
        DVec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: DVec3::new(0.5, 0.5, 0.5),
        },
    ));

    for (a, b) in
        (-11..11).cartesian_product(-11..11).into_iter()
    {
        let choose_mat = rng.gen::<f64>();
        let center = DVec3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );

        if (center - DVec3::new(4., 0.2, 0.)).length() > 0.9
        {
            let material = if choose_mat < 0.8 {
                // diffuse
                let albedo = DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                ) * DVec3::new(
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                    rng.gen_range(0f64..1.),
                );
                Material::Lambertian { albedo: albedo }
            } else if choose_mat < 0.95 {
                // metal
                let albedo = DVec3::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );
                let fuzz = rng.gen_range(0f64..0.5);

                Material::Metal { albedo, fuzz }
            } else {
                // glass
                Material::Dielectric {
                    index_of_refraction: 1.5,
                }
            };

            world.push(Sphere::new(center, 0.2, material));
        }
    }

    world.push(Sphere::new(
        DVec3::new(0., 1., 0.),
        1.0,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));

    world.push(Sphere::new(
        DVec3::new(-4., 1., 0.),
        1.0,
        Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1),
        },
    ));

    world.push(Sphere::new(
        DVec3::new(4., 1., 0.),
        1.0,
        Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));

    let camera = Camera::init()
        .image_width(400)
        .aspect_ratio(16.0 / 9.0)
        .look_from(DVec3::new(13., 2., 3.))
        .look_at(DVec3::ZERO)
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.)
        .build();

    camera.render_to_disk(
        "raytracing-in-one-weekend-final-scene",
        &*world,
    )?;

    Ok(())
}
