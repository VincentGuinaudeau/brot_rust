
use super::{ PointRenderer, View, Trace, TraceStatus };

pub struct BinaryPointRenderer {}

impl BinaryPointRenderer {
	pub fn new() -> BinaryPointRenderer {
		BinaryPointRenderer {}
	}
}

impl PointRenderer for BinaryPointRenderer {

	fn render(&self, view: &View, coords: &mut Vec< (u32, u32, u16) >, trace: &Trace) -> () {

		if trace.status() == TraceStatus::Outside {

			if let Some((x, y)) = view.translate_point_to_view_coordinate(trace.origin()) {
				coords.push((x as u32, y as u32, 1));
			}
		}
	}
}
