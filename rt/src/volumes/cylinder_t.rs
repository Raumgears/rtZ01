use std::sync::Arc;

use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{Point3, Ray, Vec3, dot, unit_vec};

pub struct CylinderTube {
    pub base: Point3,
    pub orientation: Vec3,
    pub length: f64,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl CylinderTube {
    pub fn new(base: Point3, length: f64, radius: f64, orientation: Vec3, mat: Arc<dyn Material>) -> CylinderTube {
        CylinderTube {
			base,
			length,
			radius,
			orientation: unit_vec(orientation),
			mat
		}
    }
}

impl Hittable for CylinderTube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let x = ray.origin() - self.base;
        let a = ray.direction().length_squared() - (dot(ray.direction(), self.orientation)).powf(2.0);
        let half_b = dot(ray.direction(), x) - (dot(ray.direction(), self.orientation) * dot(x, self.orientation));
        let c = x.length_squared() - dot(x, self.orientation).powf(2.0) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        let m = dot(ray.direction(), self.orientation*root) + dot(x, self.orientation);

        if m < 0.0 || m > self.length {
            return false;
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = unit_vec(rec.p - self.base - self.orientation * m);
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}