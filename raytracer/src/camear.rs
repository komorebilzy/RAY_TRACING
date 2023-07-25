use crate::utility::*;
#[derive(Clone)]
pub struct Camera {
    origin: Vect3,
    lower_left_corner: Vect3,
    horizontal: Vect3,
    vertical: Vect3,
    u: Vect3,
    v: Vect3,
    _w: Vect3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        look: (Vect3, Vect3),
        vup: Vect3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time: (f64, f64),
    ) -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        // let focal_length = 1.0;
        let ww = unit_vector(look.0 - look.1);
        let uu = unit_vector(cross(vup, ww));
        let vv = cross(ww, uu);
        Camera {
            _w: ww,
            u: uu,
            v: vv,
            origin: look.0,
            horizontal: uu * viewport_width * focus_dist,
            vertical: vv * viewport_height * focus_dist,
            lower_left_corner: look.0
                - uu * viewport_width * focus_dist / 2.0
                - vv * viewport_height * focus_dist / 2.0
                - ww * focus_dist,
            lens_radius: aperture / 2.0,
            time0: time.0,
            time1: time.1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            random_double_rng(self.time0, self.time1),
        )
    }
}
