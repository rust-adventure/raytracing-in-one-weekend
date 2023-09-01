use glam::DVec3;

use super::Camera;

pub struct CameraBuilder {
    image_width: u32,
    aspect_ratio: f64,
    look_from: DVec3,
    look_at: DVec3,
    vup: DVec3,
    focus_dist: f64,
    defocus_angle: f64,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f64,
    background: Option<DVec3>,
}
impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            look_from: DVec3::NEG_Z,
            look_at: DVec3::ZERO,
            vup: DVec3::Y,
            focus_dist: 10.,
            defocus_angle: 0.,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 20.,
            background: None,
        }
    }
}

impl CameraBuilder {
    pub fn image_width(
        mut self,
        image_width: u32,
    ) -> CameraBuilder {
        self.image_width = image_width;
        self
    }
    pub fn aspect_ratio(
        mut self,
        aspect_ratio: f64,
    ) -> CameraBuilder {
        self.aspect_ratio = aspect_ratio;
        self
    }
    pub fn look_from(
        mut self,
        look_from: DVec3,
    ) -> CameraBuilder {
        self.look_from = look_from;
        self
    }
    pub fn look_at(
        mut self,
        look_at: DVec3,
    ) -> CameraBuilder {
        self.look_at = look_at;
        self
    }
    pub fn vup(mut self, vup: DVec3) -> CameraBuilder {
        self.vup = vup;
        self
    }
    pub fn focus_dist(
        mut self,
        focus_dist: f64,
    ) -> CameraBuilder {
        self.focus_dist = focus_dist;
        self
    }
    pub fn defocus_angle(
        mut self,
        defocus_angle: f64,
    ) -> CameraBuilder {
        self.defocus_angle = defocus_angle;
        self
    }
    pub fn samples_per_pixel(
        mut self,
        samples_per_pixel: u32,
    ) -> CameraBuilder {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
    pub fn max_depth(
        mut self,
        max_depth: u32,
    ) -> CameraBuilder {
        self.max_depth = max_depth;
        self
    }
    pub fn vfov(mut self, vfov: f64) -> CameraBuilder {
        self.vfov = vfov;
        self
    }
    pub fn background(
        mut self,
        bg: DVec3,
    ) -> CameraBuilder {
        self.background = Some(bg);
        self
    }
    pub fn build(self) -> Camera {
        let max_value: u8 = 255;
        let image_height: u32 = (self.image_width as f64
            / self.aspect_ratio)
            as u32;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width: f64 = viewport_height
            * (self.image_width as f64
                / image_height as f64);

        let center: DVec3 = self.look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        // ## Calculate the vectors across the horizontal and down the vertical viewport edges.
        // Vector across viewport horizontal edge
        let viewport_u = viewport_width * u;
        // Vector down viewport vertical edge
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: DVec3 =
            viewport_u / self.image_width as f64;
        let pixel_delta_v: DVec3 =
            viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: DVec3 = center
            - self.focus_dist * w
            - viewport_u / 2.
            - viewport_v / 2.;
        let pixel00_loc: DVec3 = viewport_upper_left
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        //   no tan: 0.296705972839036
        // with tan: 0.29746145598814155
        let defocus_radius = self.focus_dist
            * (self.defocus_angle / 2.).to_radians().tan();

        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: self.image_width,
            image_height,
            max_value,
            aspect_ratio: self.aspect_ratio,
            center,
            pixel_delta_u,
            pixel_delta_v,
            // viewport_upper_left,
            pixel00_loc,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            vfov: self.vfov,
            lookfrom: self.look_from,
            lookat: self.look_at,
            vup: self.vup,
            u,
            v,
            w,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
            defocus_disk_u,
            defocus_disk_v,
            background: self.background,
        }
    }
}
