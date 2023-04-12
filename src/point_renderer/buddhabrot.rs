
use super::{ Args, PointRenderer, View, Trace, TraceStatus };

pub struct BuddhebrotPointRenderer {
	range_start: usize,
	range_stop: usize,
}

impl BuddhebrotPointRenderer {
	pub fn new(args: &Args) -> BuddhebrotPointRenderer {
		BuddhebrotPointRenderer {
			range_start: args.range_start,
			range_stop: args.range_stop,
		}
	}
}

impl PointRenderer for BuddhebrotPointRenderer {

	fn render(&self, view: &View, coords: &mut Vec< (u32, u32, u16) >, trace: &Trace) -> () {
		if
			trace.status() == TraceStatus::Outside &&
			(self.range_start..self.range_stop).contains(&trace.len())
		{
			for point in trace.iter() {
				if let Some((x, y)) = view.translate_point_to_view_coordinate(point) {
					coords.push((x as u32, y as u32, 1));
				}
			}
		}
	}
}
