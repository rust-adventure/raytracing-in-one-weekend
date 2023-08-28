use glam::DVec3;
use rand::Rng;

fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = DVec3::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );

        if vec.length_squared() < 1. {
            break vec;
        }
    }
}

pub fn random_unit_vector() -> DVec3 {
    return random_in_unit_sphere().normalize();
}

fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0
    // In the same hemisphere as the normal
    {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
