use std::rc::Rc;
use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{dot, Ray, Vec3, unit_vec};
use crate::utils::near_zero;


pub struct Plane {
    norm: Vec3,
    dist: f64,
    mat: Rc<dyn Material>,

}

// Dist should be negative
impl Plane {
    pub fn new(norm: Vec3, dist: f64, mat: Rc<dyn Material>) -> Plane {
        let unit_norm = unit_vec(norm);
        let unit_dist = dist / norm.length();
        Plane {
            norm: unit_norm,
            dist: unit_dist,
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let num = dot(self.norm, r.origin()) + self.dist;

        let denom = dot(self.norm,r.direction());
        if near_zero(denom) {
            return false;
        }

        let root = -num / denom;
        if root < t_min || root > t_max {
            return false
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, self.norm);
        rec.mat = Some(self.mat.clone());
        true
    }
}