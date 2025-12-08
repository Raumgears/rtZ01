use crate::basics::{Color, Ray, dot, rand_in_unit_sphere, rand_unit_vec, reflect, refract, unit_vec};
use crate::traits::HitRecord;
use crate::utils::rand_01;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
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