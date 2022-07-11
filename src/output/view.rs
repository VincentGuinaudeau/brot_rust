
use crate::core::point::Point;

pub struct View {

	/*
	 * initial values
	 */
	x_size:   i32,
	y_size:   i32,
	position: Point,

	/*
	 * internal computed properties
	 */
	// The distance bewteen two pixels in the complex plan
	step: f32
}

impl View {
	pub fn new (x_size: i32, y_size: i32, position: Point, scale: f32) -> View {
		let longer_size    = if x_size < y_size { y_size } else { x_size };
		View {
			x_size,
			y_size,
			position,
			step: (scale * 2.) / longer_size as f32,
		}	
	}

	pub fn x_size(&self) -> i32 { self.x_size }
	pub fn y_size(&self) -> i32 { self.y_size }

	pub fn translate_point_to_view_coordinate(&self, point: &Point) -> Option< (i32, i32) > {
		let x = ((point.r() - self.position.r()) / self.step).floor() as i32 + self.x_size / 2;
		let y = ((point.i() - self.position.i()) / self.step).floor() as i32 + self.y_size / 2;

		if (0..self.x_size).contains(&x) && (0..self.y_size).contains(&y) {
			Some((x, y))
		}
		else {
			None
		}
	}
}
