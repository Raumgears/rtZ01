use crate::traits::{HitRecord, Hittable};
use crate::basics::{dot, Point3, Ray};

pub struct Cube {
    pub center: Point3,
    pub size: f64,
}

impl Cube {
    pub fn new(center: Point3, size: f64) -> Self {
        Self { center, size }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    false
    }
}