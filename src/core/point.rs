
use std::ops;

use std::str::FromStr;

use std::fmt::Display;

pub type FractFloat = f64;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
	r: FractFloat,
	i: FractFloat,
}

impl Point {

	pub fn new(r: FractFloat, i: FractFloat) -> Point {
		Point { r, i }
	}

	pub fn new_null() -> Point {
		Point { r: 0., i: 0. }
	}

	pub fn r(&self) -> FractFloat {
		self.r
	}

	pub fn i(&self) -> FractFloat {
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

impl FromStr for Point {
	type Err = String;

	fn from_str(s: &str) -> Result<Point, Self::Err> {
		let mut r: FractFloat = 0.;
		let mut i: FractFloat = 0.;
		if s.ends_with("i") {
			let separator_index;
			if s.ends_with("+i") {
				separator_index = s.len() - 2;
				i = 1.;
			}
			else if s.ends_with("-i") {
				separator_index = s.len() - 2;
				i = -1.;
			}
			else {
				separator_index = s.rfind(|c: char| ['+', '-'].contains(&c)).unwrap_or(0);
				let imaginary_part = s.split_at(separator_index).1.split_at(s.len() - separator_index - 1).0;
				if imaginary_part.len() == 0 {
					i = 1.;
				}
				else {
					i = imaginary_part.parse::<FractFloat>().map_err(|_err| format!("Invalid imaginary part '{}'", s))?;
				}
			}

			if separator_index > 0 {
				let real_part = s.split_at(separator_index).0;
				r = real_part.parse::<FractFloat>().map_err(|_err| format!("Invalid real part '{}'", s))?;
			}
		}
		else if s.len() > 0 {
			r = s.parse::<FractFloat>().map_err(|_err| format!("Invalid real part '{}'", s))?;
		}
		else {
			return Err("No value provided".to_string());
		}

		Ok( Point { r, i } )
	}
}

impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.r != 0. || self.i == 0. {
	    	write!(f, "{}", self.r)?;
		}
		if self.i != 0. {
			if self.i > 0. {
				write!(f, "+")?;
			}
			write!(f, "{}i", self.i)?;
		}
		Ok(())
	}
}
