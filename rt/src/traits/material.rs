use crate::basics::{Color, Ray};
use crate::traits::HitRecord;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
