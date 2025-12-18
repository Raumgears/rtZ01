use crate::basics::{Color, Ray};
use crate::traits::HitRecord;

// Send + Sync necessary to work with Arc & rayon crate
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
