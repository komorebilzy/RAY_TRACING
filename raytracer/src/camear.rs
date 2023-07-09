use crate::*;
pub struct Camera {
    origin: Vect3,
    lower_left_corner: Vect3,
    horizontal: Vect3,
    vertical: Vect3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        Camera {
            origin: (Vect3::new(0.0, 0.0, 0.0)),
            horizontal: (Vect3::new(viewport_width, 0.0, 0.0)),
            vertical: (Vect3::new(0.0, viewport_height, 0.0)),
            lower_left_corner: (Vect3::new(0.0, 0.0, 0.0)
                - Vect3::new(viewport_width, 0.0, 0.0) / 2.0
                - Vect3::new(0.0, viewport_height, 0.0) / 2.0
                - Vect3::new(0.0, 0.0, focal_length)),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
