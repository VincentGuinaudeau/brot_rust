
use super::point::Point;

#[derive(PartialEq, Debug)]
pub enum TraceStatus {
	NotDone,
	Inside,
	Outside,
}

#[derive(Debug)]
pub struct Trace {
	origin:          Point,
	max_length:      usize,
	path:            Vec< Point >,
}

impl Trace {
	pub fn new(max_length: usize) -> Trace {
		Trace {
			origin:          Point::new_null(),
			max_length,
			path:            Vec::with_capacity(max_length),
		}
	}

	pub fn origin(&self) -> &Point {
		&self.origin
	}

	pub fn origin_mut(&mut self) -> &Point {
		&self.origin
	}

	pub fn tail(&self) -> &Point {
		self.path.last().unwrap_or(&self.origin)
	}

	pub fn len(&self) -> usize {
		self.path.len()
	}

	pub fn is_at_max_length(&self) -> bool {
		self.path.len() >= self.max_length
	}

	pub fn status(&self) -> TraceStatus {
		if self.tail().is_outside() {
			TraceStatus::Outside
		}
		else {
			if self.is_at_max_length() {
				TraceStatus::Inside
			}
			else {
				TraceStatus::NotDone
			}
		}
	}

	pub fn reset(&mut self, new_origin: Point) {
		self.origin = new_origin;
		self.path.clear();
	}

	pub fn extend(&mut self, point: Point) {
		assert!(self.path.len() < self.max_length, "trace is extended behond its inital length");
		self.path.push(point);
	}

	pub fn get_next_point(&mut self) -> &mut Point {
		assert!(self.path.len() < self.max_length, "trace is extended behond its inital length");
		self.path.push(Point::new_null());
		self.path.last_mut().unwrap()
	}

	pub fn iter(&self) -> std::iter::Chain<std::array::IntoIter<&Point, 1>, std::slice::Iter<Point>> {
		[&self.origin].into_iter().chain(self.path.iter())
	}

	pub fn iter_mut(&mut self) -> std::iter::Chain<std::array::IntoIter<& Point, 1>, std::slice::Iter<Point>> {
		[&self.origin].into_iter().chain(self.path.iter())
	}
}

impl<'a> std::iter::IntoIterator for &'a Trace {
	type Item = &'a Point;
	type IntoIter = std::iter::Chain<std::array::IntoIter<&'a Point, 1>, std::slice::Iter<'a, Point>>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
