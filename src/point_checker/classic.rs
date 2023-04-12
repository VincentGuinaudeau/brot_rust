
use std::collections::vec_deque::VecDeque;
use super::{
	Checker,
	View,
	Args,
	TraceInitializer,
};
use crate::point_renderer::{ select_point_renderer, PointRenderer };
use crate::core::{ trace::{ Trace, TraceStatus }, batch::PinBatch };

pub struct ClassicChecker {
	point_renderer: Box<dyn PointRenderer>,
	waiting_to_collect: VecDeque< PinBatch >,
}

impl ClassicChecker {
	pub fn new(args: &Args) -> ClassicChecker {
		ClassicChecker {
			point_renderer: select_point_renderer(args),
			waiting_to_collect: VecDeque::new(),
		}
	}
}

impl Checker for ClassicChecker {
	fn get_batch_ideal_capacity(&self) -> usize { 1 }

	fn push_batch(&mut self, view: &View, args: &Args, mut batch: PinBatch, mut trace_init: TraceInitializer ) {
		let mut trace = Trace::new(args.range_stop);
		let mut coords:Vec< (u32, u32, u16) > = vec![];

		for _ in 0..batch.size {
			trace_init(&mut trace);
			while trace.status() == TraceStatus::NotDone {
				let new_point = trace.tail().squared() + trace.origin();
				trace.extend(new_point);
			}

			self.point_renderer.as_ref().render(view, &mut coords, &trace);
		}

		batch.coords.append(&mut coords);

		self.waiting_to_collect.push_back(batch)
	}

	fn collect_batch(&mut self) -> Option< PinBatch > {
		self.waiting_to_collect.pop_front()
	}
}
