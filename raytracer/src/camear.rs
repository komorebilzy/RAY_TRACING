use crate::*;
pub struct Camera {
    origin: Vect3,
    lower_left_corner: Vect3,
    horizontal: Vect3,
    vertical: Vect3,
}

impl Camera {
    pub fn new(lookfrom: Vect3, lookat: Vect3, vup: Vect3, vfov: f64, aspect_ratio: f64) -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        // let focal_length = 1.0;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);
        Camera {
            origin: lookfrom,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
            lower_left_corner: lookfrom - u * viewport_width / 2.0 - v * viewport_height / 2.0 - w,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
