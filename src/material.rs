use crate::{color::Color, hit::Hit, random::Random, ray::Ray, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub metallic: f64,
}

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Color,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scattered> {
        let diffuse_dir = hit.normal + Vector3::random();
        let metallic_dir = reflect(&ray.direction, &hit.normal);
        Some(Scattered {
            ray: Ray::new(hit.origin, diffuse_dir.lerp(&metallic_dir, self.metallic)),
            attenuation: self.albedo,
        })
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: 0.5.into(),
            metallic: 0.0,
        }
    }
}

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    v - 2.0 * v.dot(n) * n
}
