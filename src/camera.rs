use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{hit::Hittable, ray::Ray, scene::Scene, viewport::Viewport, Color, Point3, Vector3};

pub struct Camera {
    pub location: Point3,
    pub focal_length: f64,
}

impl Camera {
    pub fn new(location: Point3, focal_length: f64) -> Camera {
        Camera {
            location,
            focal_length,
        }
    }

    pub fn render(&self, buffer: &mut [u8], viewport: &Viewport, scene: &Scene) {
        buffer
            .par_chunks_exact_mut(3)
            .enumerate()
            .for_each(|(i, pixel)| {
                let target = viewport.get_pixel_location(i);
                let ray = Ray::new(self.location, target - self.location);
                let color = trace_ray(&scene, ray);
                pixel.copy_from_slice(&color.bytes());
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
