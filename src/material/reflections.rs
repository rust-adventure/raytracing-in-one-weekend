use glam::DVec3;

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    return v - 2. * v.dot(n) * n;
}

pub fn refract(
    uv: DVec3,
    n: DVec3,
    etai_over_etat: f64,
) -> DVec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: DVec3 =
        -((1.0 - r_out_perp.length_squared()).abs()).sqrt()
            * n;
    return r_out_perp + r_out_parallel;
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}
