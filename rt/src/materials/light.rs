use crate::basics::{Color, Ray};
use crate::traits::{HitRecord, Material};

pub struct DiffuseLight {
    emit: Color,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        DiffuseLight { emit: c }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self) -> Color {
        self.emit
    }
}
