use std::rc::Rc;

use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{Point3, Ray, Vec3};

pub struct Cube {
    pub center: Point3,
    pub size: f64,
    pub mat: Rc<dyn Material>,
}

impl Cube {
    pub fn new(center: Point3, size: f64, mat: Rc<dyn Material>) -> Self {
        Self { center, size, mat }
    }

    pub fn outward_normal(&self, point: Point3) -> Vec3 {
        let x = ((point.x() - self.center.x()) / self.size) as i32;
        let y = ((point.y() - self.center.y()) / self.size) as i32;
        let z = ((point.z() - self.center.z()) / self.size) as i32;
        Vec3::new(x as f64, y as f64, z as f64)
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Slab method (AABB)
        let low_bounds = self.center + Vec3::new(-self.size, -self.size, -self.size);
        let high_bounds = self.center + Vec3::new(self.size, self.size, self.size);

        let t_low = Vec3::new((low_bounds.x() - ray.origin().x()) / ray.direction().x(),
                                    (low_bounds.y() - ray.origin().y()) / ray.direction().y(),
                                    (low_bounds.z() - ray.origin().z()) / ray.direction().z());
        let t_high = Vec3::new((high_bounds.x() - ray.origin().x()) / ray.direction().x(),
                                     (high_bounds.y() - ray.origin().y()) / ray.direction().y(),
                                     (high_bounds.z() - ray.origin().z()) / ray.direction().z());

        let t_close = Vec3::new(t_low.x().min(t_high.x()),
                                      t_low.y().min(t_high.y()),
                                      t_low.z().min(t_high.z()));
        let t_far = Vec3::new(t_low.x().max(t_high.x()),
                                    t_low.y().max(t_high.y()),
                                    t_low.z().max(t_high.z()));
        
        let tc = t_close.x().max(t_close.y().max(t_close.z()));
        let tf = t_far.x().min(t_far.y().min(t_far.z()));

        if tc <= tf && tf > 0. && tc >= t_min && tc <= t_max {
            rec.t = tc;
            rec.p = ray.at(rec.t);
            rec.set_face_normal(ray, self.outward_normal(rec.p));
            rec.mat = Some(self.mat.clone());
            return true;
        }
        false
    }
}