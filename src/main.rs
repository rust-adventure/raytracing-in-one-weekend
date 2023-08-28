use glam::DVec3;
use itertools::Itertools;
use rand::prelude::*;
use raytracer::{
    camera::{Camera, CameraNew},
    hittable::HittableList,
    material::Material,
    shapes::sphere::Sphere,
};

use std::io;
fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = HittableList { objects: vec![] };

    let ground_material = Material::Lambertian {
        albedo: DVec3::new(0.5, 0.5, 0.5),
    };
    world.add(Sphere {
        center: DVec3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    });
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
            if choose_mat < 0.8 {
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
                let material =
                    Material::Lambertian { albedo: albedo };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let albedo = DVec3::new(
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                    rng.gen_range(0.5..1.),
                );
                let fuzz = rng.gen_range(0f64..0.5);
                let material =
                    Material::Metal { albedo, fuzz };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    let material1 = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    world.add(Sphere {
        center: DVec3::new(0., 1., 0.),
        radius: 1.0,
        material: material1,
    });

    let material2 = Material::Lambertian {
        albedo: DVec3::new(0.4, 0.2, 0.1),
    };
    world.add(Sphere {
        center: DVec3::new(-4., 1., 0.),
        radius: 1.0,
        material: material2,
    });

    let material3 = Material::Metal {
        albedo: DVec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Sphere {
        center: DVec3::new(4., 1., 0.),
        radius: 1.0,
        material: material3,
    });

    let camera = Camera::new(CameraNew {
        image_width: 1200,
        aspect_ratio: 16.0 / 9.0,
        look_from: Some(DVec3::new(13., 2., 3.)),
        look_at: Some(DVec3::ZERO),
        vup: Some(DVec3::Y),
        focus_dist: Some(10.0),
        defocus_angle: Some(0.0),
    });
    camera.render_to_disk(world)?;

    Ok(())
}
