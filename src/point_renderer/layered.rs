
use super::{ PointRenderer, View, Trace, TraceStatus };

pub struct LayeredPointRenderer {}

impl LayeredPointRenderer {
	pub fn new() -> LayeredPointRenderer {
		LayeredPointRenderer {}
	}
}

impl PointRenderer for LayeredPointRenderer {

	fn render(&self, view: &View, coords: &mut Vec< (u32, u32, u16) >, trace: &Trace) -> () {

		if trace.status() == TraceStatus::Outside {

			if let Some((x, y)) = view.translate_point_to_view_coordinate(trace.origin()) {
				coords.push((x as u32, y as u32, trace.len() as u16));
			}
		}
	}
}
