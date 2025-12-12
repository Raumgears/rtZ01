use std::f64::consts::PI;
use std::rc::Rc;

use crate::traits::{HitRecord, Hittable, Material};
use crate::basics::{Point3, Ray, Vec3, rotate, unit_vec, dot};

pub struct Cube {
    pub center: Point3,
    pub size: f64,
    pub mat: Rc<dyn Material>,
    pub rotation: Vec3,
}

impl Cube {
    pub fn new(center: Point3, size: f64, rotation: Vec3, mat: Rc<dyn Material>) -> Self {
        Self { center, size, mat, rotation: rotation * (PI/180.0) }
    }

    pub fn outward_normal(&self, point: Point3) -> Vec3 {
        // let x = ((point.x() - self.center.x()) / self.size) as i32;
        // let y = ((point.y() - self.center.y()) / self.size) as i32;
        // let z = ((point.z() - self.center.z()) / self.size) as i32;
        // // if (x == y && y == z && z == 0) {
        // //     eprintln!("ERREUR NORMALE");
        // // }
        // let result = Vec3::new(x as f64, y as f64, z as f64);
        // eprintln!("x: {}, y: {}, z: {}", result.x(), result.y(), result.z());
        // result

        let normals_vec= vec![
            Vec3::new(0.0, 0.0, 1.0), //front
            Vec3::new(0.0, 0.0, -1.0), //back
            Vec3::new(0.0, 1.0, 0.0), //top
            Vec3::new(0.0, -1.0, 0.0), //bottom
            Vec3::new(1.0, 0.0, 0.0), //right
            Vec3::new(-1.0, 0.0, 0.0) //left
        ];
        let mut result_index = 0;
        let mut dot_result = 0.0;
        let collision = point - self.center;

        for (i, &v) in normals_vec.iter().enumerate() {
            let result = dot(collision, v);
            if result > dot_result {
                dot_result = result;
                result_index = i;
            }
        }

        normals_vec[result_index]
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Rotate ray origin around cube center by cube's rotation 
        let oc = ray.origin() - self.center;

        let clone_ray = Ray::new((ray.origin() + self.center + rotate(oc, self.rotation)) - ray.origin(), rotate(ray.direction(), self.rotation));
        // Slab method (AABB)
        let low_bounds = self.center + Vec3::new(-self.size, -self.size, -self.size);
        let high_bounds = self.center + Vec3::new(self.size, self.size, self.size);

        let t_low = Vec3::new((low_bounds.x() - clone_ray.origin().x()) / clone_ray.direction().x(),
                                    (low_bounds.y() - clone_ray.origin().y()) / clone_ray.direction().y(),
                                    (low_bounds.z() - clone_ray.origin().z()) / clone_ray.direction().z());
        let t_high = Vec3::new((high_bounds.x() - clone_ray.origin().x()) / clone_ray.direction().x(),
                                     (high_bounds.y() - clone_ray.origin().y()) / clone_ray.direction().y(),
                                     (high_bounds.z() - clone_ray.origin().z()) / clone_ray.direction().z());

        let t_close = Vec3::new(t_low.x().min(t_high.x()),
                                      t_low.y().min(t_high.y()),
                                      t_low.z().min(t_high.z()));
        let t_far = Vec3::new(t_low.x().max(t_high.x()),
                                    t_low.y().max(t_high.y()),
                                    t_low.z().max(t_high.z()));
        
        let tc = t_close.x().max(t_close.y().max(t_close.z()));
        let tf = t_far.x().min(t_far.y().min(t_far.z()));

        // eprintln!("orig : x: {}, y: {}, z: {}", ray.origin().x(), ray.origin().y(), ray.origin().z());
        // eprintln!("clone : x: {}, y: {}, z: {}", clone_ray.origin().x(), clone_ray.origin().y(), clone_ray.origin().z());
        if tc <= tf && tf > 0. && tc >= t_min && tc <= t_max {
            rec.t = tc;
            rec.p = ray.at(rec.t);
            rec.set_face_normal(ray, rotate(self.outward_normal(clone_ray.at(tc)), -self.rotation));
            rec.mat = Some(self.mat.clone());
            return true;
        }
        false
    }
}