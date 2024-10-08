use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::material::Material;
use raytracing::scene::Scene;
use raytracing::sphere::Sphere;
use raytracing::viewport::Viewport;
use raytracing::Point3;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SAMPLES_PER_PIXEL: usize = 16;
const MAX_BOUNCES: usize = 4;

fn main() {
    let mut buffer = vec![0u8; WIDTH * HEIGHT * 3];

    let mut scene = Scene::new();
    scene.add(Sphere::new(
        Point3::new(0.0, 0.0, -2.0),
        0.5,
        Material {
            albedo: Color::new(1.0, 0.2, 0.2),
            metallic: 0.8,
        },
    ));
    scene.add(Sphere::new(
        Point3::new(1.5, 0.0, -2.0),
        0.5,
        Material {
            albedo: Color::new(0.2, 1.0, 0.2),
            metallic: 0.8,
        },
    ));
    scene.add(Sphere::new(
        Point3::new(-1.5, 0.0, -2.0),
        0.5,
        Material {
            albedo: Color::new(0.2, 0.2, 1.0),
            metallic: 0.8,
        },
    ));
    scene.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Material::default(),
    ));

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        1.0,
        SAMPLES_PER_PIXEL,
        MAX_BOUNCES,
    );
    let viewport = Viewport::new(WIDTH, HEIGHT, &camera);

    let start_time = std::time::Instant::now();
    camera.render(&mut buffer, &viewport, &scene);
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
