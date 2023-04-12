
use crate::core::mtwister_rand::Rng;
use crate::core::trace::Trace;
use crate::core::batch::{ Batch, PinBatch };
use super::{ PointFinder, View, Slate, Args, Checker };

pub struct RandomPointFinder;

fn send_new_batch(checker: &mut dyn Checker, view: &View, args: &Args, mut batch: PinBatch, rng: &mut Rng) {
	batch.reset();
	let mut sub_rng = Rng::new(rng.u32());
	checker.push_batch(view, args, batch, Box::new(move |trace: &mut Trace| {
		trace.reset(sub_rng.point());
	}));
}

fn handle_completed_batch(slate: &mut Slate, batch: &PinBatch, good_point: &mut usize) {
	for (x, y, value) in batch.coords.iter() {
		slate.increment(*x, *y, *value);
	}
	*good_point += batch.coords.len();
}

impl PointFinder for RandomPointFinder {
	fn execute(view: &View, args: &Args, checker: &mut dyn Checker) -> Slate {
		let mut slate       = Slate::from_view(view);
		let mut rng         = Rng::new(4678059);
		let mut good_points = 0;

		let ideal_checker_capacity = checker.get_batch_ideal_capacity();

		// loading the checkers with work
		for _i in 0..ideal_checker_capacity {
			let batch = Batch::new(args.range_stop, args.batch_size);
			send_new_batch(checker, view, args, batch, &mut rng);
		}

		// if we have a fixed amount of point to check
		if args.point_sample > 0 {
			// The number of rounds is known
			for _i in 0..(args.point_sample / args.batch_size + 1) {
				let batch = checker.collect_batch().unwrap();
				handle_completed_batch(&mut slate, &batch, &mut good_points);
				send_new_batch(checker, view, args, batch, &mut rng);
			}
		}
		// else if we has to find a fixed number of points
		else if args.point_target > 0 {
			// We have to loop until we find the specified amount of points
			while good_points < args.point_target {
				let batch = checker.collect_batch().unwrap();
				handle_completed_batch(&mut slate, &batch, &mut good_points);
				send_new_batch(checker, view, args, batch, &mut rng);
			}
			println!("{good_points}");
		}
		else {
			panic!("no objective");
		}

		checker.done();
		
		// collecting remaining work
		while let Some( batch ) = checker.collect_batch() {
			handle_completed_batch(&mut slate, &batch, &mut good_points);
		}

		slate
	}
}
