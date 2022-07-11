
use std::ops;

// #[derive(Copy, Clone)]
pub struct Point {
	r: f32,
	i: f32,
}

impl Point {

	pub fn new(r: f32, i: f32) -> Point {
		Point { r, i }
	}

	pub fn new_null() -> Point {
		Point { r: 0., i: 0. }
	}

	pub fn r(&self) -> f32 {
		self.r
	}

	pub fn i(&self) -> f32 {
		self.i
	}

	pub fn squared(&self) -> Point {
		Point {
			r: self.r * self.r - self.i * self.i,
			i: 2. * self.r * self.i,
		}
	}

	pub fn is_inside(&self) -> bool {
		self.r * self.r + self.i * self.i < 4.
	}

	pub fn is_outside(&self) -> bool {
		!self.is_inside()
	}
}

impl ops::Add for Point {
	type Output = Point;
	fn add(self, other: Point) -> Point {
		Point { r: self.r + other.r, i: self.i + other.i }
	}
}
impl ops::Add<&Point> for Point {
	type Output = Point;
	fn add(self, other: &Point) -> Point {
		Point { r: self.r + other.r, i: self.i + other.i }
	}
}
impl ops::Add for &Point {
	type Output = Point;
	fn add(self, other: &Point) -> Point {
		Point { r: self.r + other.r, i: self.i + other.i }
	}
}
impl ops::AddAssign for Point {
	fn add_assign(&mut self, other: Point) {
		self.r += other.r;
		self.i += other.i;
	}
}

impl ops::Mul for Point {
	type Output = Point;
	fn mul(self, other: Point) -> Point {
		Point {
			r: self.r * other.r - self.i * other.i,
			i: self.r * other.i + self.i * other.r,
		}
	}
}
impl ops::Mul for &Point {
	type Output = Point;
	fn mul(self, other: &Point) -> Point {
		Point {
			r: self.r * other.r - self.i * other.i,
			i: self.r * other.i + self.i * other.r,
		}
	}
}
impl ops::MulAssign for Point {
	fn mul_assign(&mut self, other: Point) {
		let tmp = self.r * other.i + self.i * other.r;
		self.r  = self.r * other.r - self.i * other.i;
		self.i  = tmp;
	}
}
