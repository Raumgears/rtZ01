use std::sync::Arc;
use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{Point3, Ray, Vec3, dot, unit_vec};
use crate::utils::near_zero;

pub struct Disk {
    norm: Vec3,
    dist: f64,
    center: Point3,          // Proj of the point given
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Disk {
    pub fn new(norm: Vec3, dist: f64, point: Point3, radius: f64, mat: Arc<dyn Material>) -> Disk {
        let unit_norm = unit_vec(norm);
        let unit_dist = dist / norm.length();

		let signed_dist_center = dot(unit_norm, point) + unit_dist;
        let center = point - signed_dist_center * unit_norm;

        Disk {
            norm: unit_norm,
            dist: unit_dist,
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Disk {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let num = dot(self.norm, r.origin()) + self.dist;

        let denom = dot(self.norm, r.direction());
        if near_zero(denom) {
            return false;
        }

        let root = -num / denom;
        if root < t_min || root > t_max {
            return false;
        }

        let p = r.at(root);
        let v = p - self.center;
        if v.length_squared() > self.radius * self.radius {
            return false;
        }

        rec.t = root;
        rec.p = p;
        rec.set_face_normal(r, self.norm);
        rec.mat = Some(self.mat.clone());
        true
    }
}