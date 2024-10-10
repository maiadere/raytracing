use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use std::ops::Div;

use crate::{hit::Hittable, ray::Ray, scene::Scene, viewport::Viewport, Color, Point3};

pub struct Camera {
    pub location: Point3,
    pub focal_length: f64,
    pub samples_per_pixel: usize,
}

impl Camera {
    pub fn new(location: Point3, focal_length: f64, samples_per_pixel: usize) -> Camera {
        Camera {
            location,
            focal_length,
            samples_per_pixel,
        }
    }

    pub fn render(&self, buffer: &mut [u8], viewport: &Viewport, scene: &Scene) {
        buffer
            .par_chunks_exact_mut(3)
            .enumerate()
            .for_each(|(i, pixel)| {
                let color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let target = viewport.get_pixel_location(i);
                        let ray = Ray::new(self.location, target - self.location);
                        trace_ray(&scene, ray)
                    })
                    .sum::<Color>()
                    .div(self.samples_per_pixel as f64)
                    .bytes();

                pixel.copy_from_slice(&(color));
            });
    }
}

fn trace_ray(scene: &Scene, ray: Ray) -> Color {
    if let Some(hit) = scene.hit(&ray, 0.0, f64::INFINITY) {
        return 0.5 * (Color::from(hit.normal) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.0), t)
}
