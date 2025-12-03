use crate::basics::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    ori: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(ori: Point3, dir: Vec3) -> Ray {
        Ray {
            ori,
            dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.ori
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.ori + t * self.dir
    }
}