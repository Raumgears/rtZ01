use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::{rand_01, rand_range};

// Unified struct and methods for the 3 types, easy to loop on, no overwriting a field
#[derive(Copy,Clone, Default)]
pub struct Vec3 {
	tab: [f64; 3],
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vec3 { tab: [x, y, z]}
	}
	pub fn rand() -> Self {
        Self::new(
        	rand_01(),
            rand_01(),
            rand_01(),
        )
    }
    pub fn rand_range(min: f64, max: f64) -> Self {
        Self::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }


	pub fn x(&self) -> f64 {
		self.tab[0]
	}
	pub fn y(&self) -> f64 {
		self.tab[1]
	}
	pub fn z(&self) -> f64 {
		self.tab[2]
	}

	// In order to avoid using sqrt which is a long operation, we try to use squared as much as possible
	pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.tab[0] * self.tab[0] + self.tab[1] * self.tab[1] + self.tab[2] * self.tab[2]
    }

	// In order to avoid Vec null in scatter() in material.rs
	pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.x().abs() < EPS && self.y().abs() < EPS && self.z().abs() < EPS
    }
}

// Output formatting
impl Display for Vec3 {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} {} {}", self.tab[0], self.tab[1], self.tab[2])
    }
}

// Basic operations with assign version
impl Neg for Vec3 {
	type Output = Vec3;

    fn neg(self) -> Vec3 {
		Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
	}
}
impl AddAssign for Vec3 {
	fn add_assign(&mut self, v: Vec3) {
		*self = *self + v;
    }
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
	}
}
impl SubAssign for Vec3 {
	fn sub_assign(&mut self, v: Vec3){
		*self = *self - v;
	}
}

impl Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
	}
}
impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, v: Vec3) -> Vec3 {
		Vec3::new(self * v.x(), self * v.y(), self * v.z())
	}
}
impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, t: f64) -> Vec3 {
		Vec3::new(self.x() * t, self.y() * t, self.z() * t)
	}
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, t: f64) -> Vec3 {
		Vec3::new(self.x() / t, self.y() / t, self.z() / t)
	}
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x() / v.x(), self.y() / v.y(), self.z() / v.z())
	}
}
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        *self = *self / v;
    }
}


//----------------------------------------------------


pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.tab[0] * v.tab[0] + u.tab[1] * v.tab[1] + u.tab[2] * v.tab[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.tab[1] * v.tab[2] - u.tab[2] * v.tab[1],
        u.tab[2] * v.tab[0] - u.tab[0] * v.tab[2],
        u.tab[0] * v.tab[1] - u.tab[1] * v.tab[0],
    )
}


//----------------------

pub fn unit_vec(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn rand_unit_vec() -> Vec3 {
	unit_vec(rand_in_unit_sphere())
}

pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::rand_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0); // Clamp à 1.0
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}
