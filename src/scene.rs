use crate::{
    hit::{Hit, Hittable},
    ray::Ray,
};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }

    pub fn add_boxed(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
