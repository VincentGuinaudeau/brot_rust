
// use fastrand::Rng;

use crate::core::mtwister_rand::Rng;
use crate::core::point::Point;
use crate::core::batch::{ Batch, PinBatch };
use super::{ PointFinder, View, Slate, SearchParameters, Checker };

const LOOPS:usize = 1_000_000;

pub struct RandomPointFinder ;

fn send_new_batch(checker: &mut dyn Checker, view: &View, search_param: &SearchParameters, mut batch: PinBatch, rng: &mut Rng) {
	batch.reset(|rng| { Point::new( rng.f64() * 4. - 2., rng.f64() * 4. - 2. ) }, rng);
	checker.push_batch(view, search_param, batch);
}

fn handle_completed_batch(slate: &mut Slate, batch: &PinBatch) {
	for (x, y) in batch.coords.iter() {
		slate.increment(*x, *y);
	}
}

impl PointFinder for RandomPointFinder {
	fn execute(view: &View, search_param: &SearchParameters, checker: &mut dyn Checker) -> Slate {
		let mut slate = Slate::from_view(view);
		let mut rng   = Rng::new(4678059);

		let ideal_checker_capacity = checker.get_batch_ideal_capacity();

		for _i in 0..ideal_checker_capacity {
			let batch = Batch::new(search_param.upper_bound);
			send_new_batch(checker, view, search_param, batch, &mut rng);
		}

		for _i in 0..LOOPS {
			let batch = checker.collect_batch();
			handle_completed_batch(&mut slate, &batch);
			send_new_batch(checker, view, search_param, batch, &mut rng);
		}

		for _i in 0..ideal_checker_capacity {
			let batch = checker.collect_batch();
			handle_completed_batch(&mut slate, &batch);
		}

		checker.done();
		slate
	}
}
