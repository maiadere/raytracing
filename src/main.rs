use raytracing::hit::Hittable;
use raytracing::ray::Ray;
use raytracing::scene::Scene;
use raytracing::sphere::Sphere;
use raytracing::Color;
use raytracing::{Point3, Vector3};

const WIDTH: usize = 1920 / 2;
const HEIGHT: usize = 1080 / 2;
const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;

fn trace_ray(scene: &Scene, ray: Ray) -> Color {
    if let Some(hit) = scene.hit(&ray, 0.0, f64::INFINITY) {
        return 0.5 * (Color::from(hit.normal) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.0), t)
}

fn main() {
    let mut scene = Scene::new();
    scene.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    scene.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let camera_origin = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / WIDTH as f64;
    let pixel_delta_v = viewport_v / HEIGHT as f64;

    let upper_left_corner =
        camera_origin - viewport_u / 2.0 - viewport_v / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let first_pixel = upper_left_corner + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    let mut buffer = vec![0u8; WIDTH * HEIGHT * 3];

    let start_time = std::time::Instant::now();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_origin = first_pixel + x as f64 * pixel_delta_u + y as f64 * pixel_delta_v;
            let ray = Ray::new(camera_origin, pixel_origin - camera_origin);
            let color = trace_ray(&scene, ray);

            let index = (y * WIDTH + x) * 3;
            buffer[index] = color.red();
            buffer[index + 1] = color.green();
            buffer[index + 2] = color.blue();
        }
    }

    println!("render time: {:?}", start_time.elapsed());

    image::save_buffer(
        "output.png",
        &buffer,
        WIDTH as u32,
        HEIGHT as u32,
        image::ExtendedColorType::Rgb8,
    )
    .unwrap();
}
