use crate::basics::{Color, Ray, rand_unit_vec};
use crate::traits::{HitRecord, Material};

// Any mat with diffuse reflection
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
		// Diffuse Reflection (Tangential sphere)
        let mut scatter_direction = rec.normal + rand_unit_vec();

		// Exclude Vec almost null case
		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}