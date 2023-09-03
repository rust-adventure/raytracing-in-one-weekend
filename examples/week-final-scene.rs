use glam::DVec3;
use itertools::Itertools;
use noise::Perlin;
use rand::Rng;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{
        constant_medium::ConstantMedium, quad::Quad,
        quad_box::QuadBox, sphere::Sphere, Shapes,
    },
    textures::Texture,
};
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = vec![];

    let ground = Material::Lambertian {
        albedo: DVec3::new(0.48, 0.83, 0.53).into(),
    };

    let boxes_per_side = 20;
    for (j, i) in (0..boxes_per_side)
        .cartesian_product(0..boxes_per_side)
    {
        let w = 100.0;
        let x0 = -1000.0 + i as f64 * w;
        let z0 = -1000.0 + j as f64 * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = rng.gen_range(1f64..101.);
        let z1 = z0 + w;

        world.push(Shapes::QuadBox(QuadBox::new(
            DVec3::new(x0, y0, z0),
            DVec3::new(x1, y1, z1),
            ground.clone(),
        )));
    }

    let light = Material::DiffuseLight(
        DVec3::new(7., 7., 7.).into(),
    );
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(123., 554., 147.),
        DVec3::new(300., 0., 0.),
        DVec3::new(0., 0., 265.),
        light,
    )));

    let center1 = DVec3::new(400., 400., 200.);
    let center2 = center1 + DVec3::new(30., 0., 0.);
    let sphere_material = Material::Lambertian {
        albedo: DVec3::new(0.7, 0.3, 0.1).into(),
    };
    world.push(Shapes::Sphere(
        Sphere::new(center1, 50., sphere_material)
            .with_move_to(center2),
    ));

    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(260., 150., 45.),
        50.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    )));
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(0., 150., 145.),
        50.,
        Material::Metal {
            albedo: DVec3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        },
    )));

    let boundary = Shapes::Sphere(Sphere::new(
        DVec3::new(360., 150., 145.),
        70.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));
    world.push(boundary);
    let boundary = Shapes::Sphere(Sphere::new(
        DVec3::new(360., 150., 145.),
        70.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));
    world.push(Shapes::ConstantMedium(
        ConstantMedium::new(
            boundary,
            0.2,
            DVec3::new(0.2, 0.4, 0.9).into(),
        ),
    ));
    let boundary = Shapes::Sphere(Sphere::new(
        DVec3::new(0., 0., 0.),
        5000.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));
    world.push(Shapes::ConstantMedium(
        ConstantMedium::new(
            boundary,
            0.0001,
            DVec3::new(1., 1., 1.).into(),
        ),
    ));

    let earth_texture =
        Texture::load_image("assets/earthmap.jpg")?;

    let emat = Material::Lambertian {
        albedo: earth_texture,
    };
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let perlin = Perlin::new(1);
    let pertext = Texture::PerlinNoise(perlin, 0.1);
    world.push(Shapes::Sphere(Sphere::new(
        DVec3::new(220., 280., 300.),
        80.,
        Material::Lambertian { albedo: pertext },
    )));

    let white = Material::Lambertian {
        albedo: DVec3::new(0.73, 0.73, 0.73).into(),
    };
    // let mut boxes2 = vec![];

    // for _ in 0..1000 {
    //     let splat: f64 = rng.gen_range(0f64..165.);
    //     boxes2.push(Shapes::Sphere(Sphere::new(
    //         DVec3::splat(splat),
    //         10.,
    //         white.clone(),
    //     )));
    // }

    // let box2 = Shapes::Translate {
    //     offset: DVec3::new(-100., 270., 395.),
    //     object: Box::new(Shapes::new_rotate_y(
    //         15.,
    //         Shapes::Collection(boxes2),
    //     )),
    // };
    // world.push(box2);

    let camera = Camera::init()
        .image_width(800)
        .aspect_ratio(1.)
        .look_from(DVec3::new(478., 278., -600.))
        .look_at(DVec3::new(278., 278., 0.))
        .vup(DVec3::Y)
        .focus_dist(10.0)
        .defocus_angle(0.0)
        .samples_per_pixel(2000)
        .max_depth(40)
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
