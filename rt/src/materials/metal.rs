use crate::basics::{Color, Ray, dot, rand_in_unit_sphere, reflect, unit_vec};
use crate::traits::{HitRecord, Material};

pub struct Metal {
    albedo: Color,
	fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Metal {
			albedo: a,
			fuzz: if f < 1.0 {
					f
				} else {
					1.0
				}
			}
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vec(r_in.direction()), rec.normal);

        *attenuation = self.albedo;
		*scattered = Ray::new(rec.p, reflected + self.fuzz * rand_in_unit_sphere());

        dot(scattered.direction(), rec.normal) > 0.0
    }
}