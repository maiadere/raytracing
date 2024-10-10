use crate::{material::Material, ray::Ray, Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Hit {
    pub origin: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl Hit {
    pub fn new(ray: &Ray, origin: Point3, normal: Vector3, t: f64, material: Material) -> Hit {
        let front_face = normal.dot(&ray.direction) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Hit {
            origin,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
