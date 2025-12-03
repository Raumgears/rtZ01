use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Unified struct and methods for the 3 types, easy to loop on, no overwriting a field
#[derive(Copy,Clone, Default)]
pub struct Vec3 {
	tab: [f64; 3],
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vec3 { tab: [x, y, z]}
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

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}