
use std::cmp::min;
use crate::core::trace::Trace;
use crate::core::batch::{ Batch, PinBatch };
use super::{ PointFinder, View, Slate, Args, Checker };

pub struct ScanPointFinder;

fn send_new_batch(checker: &mut dyn Checker, view: &View, args: &Args, mut batch: PinBatch, mut offset: i32) -> i32 {
	let number_of_pixel = min(batch.size, (view.x_size() * view.y_size() - offset) as usize);
	batch.reset();
	let cloned_view = view.clone();
	checker.push_batch(view, args, batch, Box::new(move |trace: &mut Trace| {
		let x = offset % cloned_view.x_size();
		let y = offset / cloned_view.x_size();
	 	trace.reset(cloned_view.translate_view_coordinate_to_point(x, y));

		// if offset < 520 {
		// 	println!("{offset}, {x} {y}, {}", trace.origin());
		// }
	 	offset += 1;
	}));
	offset + number_of_pixel as i32
}

fn handle_completed_batch(slate: &mut Slate, batch: &PinBatch) {
	for (x, y, value) in batch.coords.iter() {
		slate.increment(*x, *y, *value);
	}
}

impl PointFinder for ScanPointFinder {

	fn execute(view: &View, args: &Args, checker: &mut dyn Checker) -> Slate {
		let mut slate      = Slate::from_view(view);
		let mut offset:i32 = 0;

		let ideal_checker_capacity = checker.get_batch_ideal_capacity();

		for _i in 0..ideal_checker_capacity {
			let batch = Batch::new(args.range_stop, args.batch_size);
			offset = send_new_batch(checker, view, args, batch, offset);

			if offset >= view.x_size() * view.y_size() { break; }
		}

		while offset < view.x_size() * view.y_size() {
			let batch = checker.collect_batch().unwrap();
			handle_completed_batch(&mut slate, &batch);
			offset = send_new_batch(checker, view, args, batch, offset);
		}

		checker.done();

		while let Some( batch ) = checker.collect_batch() {
			handle_completed_batch(&mut slate, &batch);
		}

		slate
	}
}
