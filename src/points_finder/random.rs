
use rand::Rng;

use crate::core::point::Point;
use crate::core::trace::{ Trace, TraceStatus };
use super::{ PointFinder, View, Slate, SearchParameters, Checker };

pub struct RandomPointFinder ;


impl PointFinder for RandomPointFinder {
	fn execute(view: &View, search_param: &SearchParameters, checker: &mut dyn Checker) -> Slate {
		let mut slate = Slate::from_view(view);


		let batch_size = 1000;
		let mut batch = Vec::with_capacity(batch_size);
		for _i in 0..batch_size {
			batch.push(Trace::new(search_param.upper_bound));
		}

		let mut rng = rand::thread_rng();

		for _i in 0..10000 {
			for trace in batch.iter_mut() {
				trace.reset(Point::new( rng.gen_range(-2.0..2.), rng.gen_range(-2.0..2.) ));
			}

			checker.push_batch(batch);
			batch = checker.collect_batch().unwrap();

			for trace in batch.iter() {
				if trace.status() == TraceStatus::Outside && (search_param.lower_bound..search_param.upper_bound).contains(&trace.len()) {
					// if let Some((x, y)) = view.translate_point_to_view_coordinate(trace.origin()) {
					// 	slate.increment(x as u32, y as u32);
					// }
					for point in trace {
						if let Some((x, y)) = view.translate_point_to_view_coordinate(point) {
							slate.increment(x as u32, y as u32);
						}
					}
				}
			}
		}

		slate
	}
}
