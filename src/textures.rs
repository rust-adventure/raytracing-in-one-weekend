use glam::DVec3;

#[derive(Clone)]
pub enum Texture {
    SolidColor(DVec3),
    Checkered { even: DVec3, odd: DVec3, scale: f64 },
}
impl Texture {
    pub fn color(
        &self,
        u: f64,
        v: f64,
        point: DVec3,
    ) -> DVec3 {
        match self {
            Texture::SolidColor(color) => *color,
            Texture::Checkered { even, odd, scale } => {
                let x_integer = (scale.recip() * point.x)
                    .floor()
                    as i32;
                let y_integer = (scale.recip() * point.y)
                    .floor()
                    as i32;
                let z_integer = (scale.recip() * point.z)
                    .floor()
                    as i32;

                let is_even =
                    (x_integer + y_integer + z_integer) % 2
                        == 0;

                if is_even {
                    *even
                } else {
                    *odd
                }
            }
        }
    }
}

impl From<DVec3> for Texture {
    fn from(value: DVec3) -> Self {
        Self::SolidColor(value)
    }
}
