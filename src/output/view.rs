
use crate::core::point::Point;

pub struct View {

	/*
	 * initial values
	 */
	x_size:   i32,
	y_size:   i32,
	position: Point,
	scale:    f32,

	/*
	 * internal computed properties
	 */
	smaller_size: i32,
	longer_size: i32,
	// The maxium distance around `position` a point can be to be in the view, squared to avoid doing a sqrt when computing
	squared_radius: f32,
	// The distance bewteen two pixels in the complex plan
	step: f32
}

impl View {
	pub fn new (x_size: i32, y_size: i32, position: Point, scale: f32) -> View {
		let smaller_size   = if x_size < y_size { x_size } else { y_size };
		let longer_size    = if x_size < y_size { y_size } else { x_size };
		let ratio          = (smaller_size as f32) / (longer_size as f32);
		let smaller_scale  = scale * ratio;
		View {
			x_size,
			y_size,
			position,
			scale,
			smaller_size,
			longer_size,
			squared_radius: scale * scale + smaller_scale * smaller_scale,
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
