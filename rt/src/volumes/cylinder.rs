use std::sync::Arc;

use crate::basics::{Point3, Vec3, dot, unit_vec};
use crate::traits::{Hittable, Material};
use crate::volumes::{CylinderTube, Disk, HittableList};

pub struct Cylinder {
    parts: HittableList
}

impl Cylinder {
    pub fn new(base: Point3, length: f64, radius: f64, orientation: Vec3, mat: Arc<dyn Material>) -> Cylinder {
        let tube = CylinderTube::new(base, length, radius, orientation, mat.clone());
        let axis = unit_vec(orientation);

        let base_disk = Disk::new(-axis, -dot(-axis, base), base, radius, mat.clone());
        let top_center = base + axis * length;
        let top_disk= Disk::new(axis, -dot(axis, top_center), top_center, radius, mat.clone());

        let mut parts: HittableList = Default::default();
        parts.add(Box::new(tube));
        parts.add(Box::new(base_disk));
        parts.add(Box::new(top_disk));
        Cylinder {
            parts
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &crate::basics::Ray, t_min: f64, t_max: f64, rec: &mut crate::traits::HitRecord) -> bool {
        self.parts.hit(ray, t_min, t_max, rec)
    }
}