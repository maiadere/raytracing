use crate::{
    hit::{Hit, Hittable},
    ray::Ray,
    Point3,
};

pub struct Sphere {
    pub location: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        let radius = radius.max(0.0);
        Sphere {
            location: center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = self.location - ray.origin;
        let a = ray.direction.norm_squared();
        let h = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let root = (h - sqrt_d) / a;

        if root <= t_min || root >= t_max {
            let root = (h + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let t = root;
        let origin = ray.at(t);
        let normal = (origin - self.location) / self.radius;

        Some(Hit::new(&ray, origin, normal, t))
    }
}
