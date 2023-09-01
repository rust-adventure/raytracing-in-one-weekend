use std::{io, path::Path};

use glam::{DVec3, Vec3Swizzles};
use image::{DynamicImage, GenericImageView};
use noise::{NoiseFn, Perlin, Turbulence};

#[derive(Clone)]
pub enum Texture {
    SolidColor(DVec3),
    Checkered { even: DVec3, odd: DVec3, scale: f64 },
    Image(DynamicImage),
    PerlinNoise(Perlin, f64),
    Turbulence(Perlin),
}
impl Texture {
    pub fn load_image<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        use image::io::Reader as ImageReader;

        let img =
            ImageReader::open(path)?.decode().unwrap();

        Ok(Self::Image(img))
    }
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
            Texture::Image(image) => {
                // If we have no texture data, then return solid cyan as a debugging aid.
                if image.height() <= 0 {
                    return DVec3::new(0., 1., 1.);
                }
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

                let i: u32 =
                    (u * image.width() as f64) as u32;
                let j: u32 =
                    (v * image.height() as f64) as u32;

                let pixel = image.get_pixel(i, j);

                let color_scale = 1.0 / 255.0;
                return DVec3::new(
                    color_scale * pixel[0] as f64,
                    color_scale * pixel[1] as f64,
                    color_scale * pixel[2] as f64,
                );
            }
            Texture::PerlinNoise(noise, freq) => {
                DVec3::ONE
                    * noise.get(
                        ((point * *freq) + 1.0 / 2.0)
                            .xyz()
                            .to_array(),
                    )
            }
            Texture::Turbulence(perlin) => {
                let noise =
                    Turbulence::<_, Perlin>::new(perlin);
                DVec3::ONE
                    * noise.get((point).xyz().to_array())
                    + 1.0 / 2.0
            }
        }
    }
}

impl From<DVec3> for Texture {
    fn from(value: DVec3) -> Self {
        Self::SolidColor(value)
    }
}
