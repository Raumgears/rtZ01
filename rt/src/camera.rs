use crate::{basics::{Point3, Ray, Vec3, cross, unit_vec}, utils::degrees_to_radians};

pub struct Camera {
	pub ori: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
}

impl Camera {
	pub fn new(aspect_ratio: f64, vangle_fov: f64, from: Point3, to: Point3, y: Vec3) -> Camera {
		let theta = degrees_to_radians(vangle_fov);
		let h = f64::tan(theta / 2.0);

		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;
		//let focal_length = 1.0;

		let w = unit_vec(from - to);
		let u = unit_vec(cross(y, w));
		let v = cross(w, u);

		let origin = from;
		let horizontal = viewport_width * u;
		let vertical = viewport_height * v;
		let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

		Camera { ori: origin,
			lower_left_corner,
			horizontal,
			vertical }
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.ori,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.ori,
        )
    }
}
