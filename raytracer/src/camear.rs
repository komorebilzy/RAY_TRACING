use crate::*;
pub struct Camera {
    origin: Vect3,
    lower_left_corner: Vect3,
    horizontal: Vect3,
    vertical: Vect3,
    u: Vect3,
    v: Vect3,
    _w: Vect3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vect3,
        lookat: Vect3,
        vup: Vect3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        // let focal_length = 1.0;
        let ww = unit_vector(lookfrom - lookat);
        let uu = unit_vector(cross(vup, ww));
        let vv = cross(ww, uu);
        Camera {
            _w: ww,
            u: uu,
            v: vv,
            origin: lookfrom,
            horizontal: uu * viewport_width * focus_dist,
            vertical: vv * viewport_height * focus_dist,
            lower_left_corner: lookfrom
                - uu * viewport_width * focus_dist / 2.0
                - vv * viewport_height * focus_dist / 2.0
                - ww * focus_dist,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
