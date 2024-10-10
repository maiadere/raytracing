use crate::random::RandomInRange;
use crate::{camera::Camera, Point3, Vector3};

pub struct Viewport {
    row_pitch: usize,
    first_pixel: Point3,
    pixel_offset: Vector3,
}

impl Viewport {
    pub fn new(width: usize, height: usize, camera: &Camera) -> Viewport {
        let row_pitch = width;
        let width = width as f64;
        let height = height as f64;

        let aspect_ratio = width / height;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_offset_u = viewport_u / width;
        let pixel_offset_v = viewport_v / height;

        let upper_left_corner = camera.location
            - viewport_u / 2.0
            - viewport_v / 2.0
            - Vector3::new(0.0, 0.0, camera.focal_length);

        let pixel_offset = pixel_offset_u + pixel_offset_v;
        let first_pixel = upper_left_corner + pixel_offset / 2.0;

        Viewport {
            row_pitch,
            first_pixel,
            pixel_offset,
        }
    }

    pub fn get_pixel_location(&self, i: usize) -> Point3 {
        let x = (i % self.row_pitch) as f64 + f64::random_in_range(-0.5, 0.5);
        let y = (i / self.row_pitch) as f64 + f64::random_in_range(-0.5, 0.5);
        self.first_pixel + Vector3::new(self.pixel_offset.x * x, self.pixel_offset.y * y, 0.0)
    }
}
