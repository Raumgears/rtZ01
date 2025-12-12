use std::sync::Arc;
use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{Point3, Ray, Vec3, cross, dot, unit_vec};
use crate::utils::{degrees_to_radians, near_zero};

pub struct Square {
    norm: Vec3,
    dist: f64,
    proj: Point3,          // Proj of the point given
	size: f64,
	base_u: Vec3,
	base_v: Vec3,
    mat: Arc<dyn Material>,
}

impl Square {
    pub fn new(norm: Vec3, dist: f64, point: Point3, size: f64, angle: f64, mat: Arc<dyn Material>) -> Square {
        let unit_norm = unit_vec(norm);
        let unit_dist = dist / norm.length();

		let signed_dist_proj = dot(unit_norm, point) + unit_dist;
        let proj = point - signed_dist_proj * unit_norm;

		let angle_rad = degrees_to_radians(angle);

		// Creating a base u, v on the plane
		let temp = if unit_norm.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let u0 = unit_vec(cross(unit_norm, temp));
        let v0 = cross(unit_norm, u0);

        // Apply rotation in the plane
        let cos_t = angle_rad.cos();
        let sin_t = angle_rad.sin();

        let u = cos_t * u0 + sin_t * v0;
        let v = -sin_t * u0 + cos_t * v0;

        Square {
            norm: unit_norm,
            dist: unit_dist,
			proj,
			size,
			base_u: u,
			base_v: v,
            mat,
        }
    }
}

impl Hittable for Square {
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

		let rel = p - self.proj;
        let x = dot(rel, self.base_u);
        let y = dot(rel, self.base_v);

        if x.abs() > self.size || y.abs() > self.size {
            return false;
        }

        rec.t = root;
        rec.p = p;
        rec.set_face_normal(r, self.norm);
        rec.mat = Some(self.mat.clone());
        true
    }
}