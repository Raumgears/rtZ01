use crate::{basics::{Color, Ray, dot, rand_in_unit_sphere, reflect, refract, unit_vec}, traits::{HitRecord, Material}, utils::rand_01};



pub struct Dielectric {
    ir: f64, // Index of refraction
	fuzz: f64,
}

impl Dielectric {
    pub fn new(ir: f64, f: f64) -> Dielectric {
        Dielectric {
            ir: ir,
			fuzz: if f < 1.0 {
				f
			} else {
				1.0
			}
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vec(r_in.direction());

		let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, self.ir) > rand_01() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(rec.p, direction + self.fuzz * rand_in_unit_sphere());
        true
    }
}

fn reflectance(
	cos_theta: f64,
	ref_idx: f64,
) -> f64 {
	let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
	r0 *= r0;
	r0 + (1.0 - r0) * f64::powf(1.0 - cos_theta, 5.0)
}