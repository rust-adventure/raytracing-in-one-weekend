use glam::DVec3;
use raytracer::{
    camera::Camera,
    material::Material,
    shapes::{
        constant_medium::ConstantMedium, quad::Quad,
        quad_box::QuadBox, Shapes,
    },
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
        DVec3::new(7., 7., 7.).into(),
    );

    world.push(Shapes::Quad(Quad::new(
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 555., 0.),
        DVec3::new(0., 0., 555.),
        green,
    )));
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(0., 0., 0.),
        DVec3::new(0., 555., 0.),
        DVec3::new(0., 0., 555.),
        red,
    )));
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(113., 554., 127.),
        DVec3::new(330., 0., 0.),
        DVec3::new(0., 0., 305.),
        light,
    )));
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(0., 555., 0.),
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(0., 0., 0.),
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.push(Shapes::Quad(Quad::new(
        DVec3::new(0., 0., 555.),
        DVec3::new(555., 0., 0.),
        DVec3::new(0., 555., 0.),
        white.clone(),
    )));

    let box1 = Shapes::Translate {
        offset: DVec3::new(265., 0., 295.),
        object: Box::new(Shapes::new_rotate_y(
            15.,
            Shapes::QuadBox(QuadBox::new(
                DVec3::new(0., 0., 0.),
                DVec3::new(165., 330., 165.),
                white.clone(),
            )),
        )),
    };
    let box2 = Shapes::Translate {
        offset: DVec3::new(130., 0., 65.),
        object: Box::new(Shapes::new_rotate_y(
            -18.,
            Shapes::QuadBox(QuadBox::new(
                DVec3::new(0., 0., 0.),
                DVec3::new(165., 165., 165.),
                white,
            )),
        )),
    };

    world.push(Shapes::ConstantMedium(
        ConstantMedium::new(
            box1,
            0.01,
            DVec3::new(0., 0., 0.).into(),
        ),
    ));
    world.push(Shapes::ConstantMedium(
        ConstantMedium::new(
            box2,
            0.01,
            DVec3::new(1., 1., 1.).into(),
        ),
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
