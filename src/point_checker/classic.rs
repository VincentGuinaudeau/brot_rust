
use std::collections::vec_deque::VecDeque;
use super::{ Checker, View, Args, TraceInitializer };
use crate::core::{ trace::TraceStatus, batch::PinBatch };

pub struct ClassicChecker {
	waiting_to_collect: VecDeque< PinBatch >,
}

impl ClassicChecker {
	pub fn new() -> ClassicChecker {
		ClassicChecker {
			waiting_to_collect: VecDeque::new(),
		}
	}
}

impl Checker for ClassicChecker {
	fn get_batch_ideal_capacity(&self) -> usize { 1 }

	fn push_batch(&mut self, view: &View, args: &Args, mut batch: PinBatch, mut trace_init: TraceInitializer ) {
		let mut coords:Vec< (u32, u32) > = vec![];
		for trace in batch.traces.iter_mut() {
			let trace = trace_init(trace);
			while trace.status() == TraceStatus::NotDone {
				let new_point = trace.tail().squared() + trace.origin();
				trace.extend(new_point);
			}

			if 
				trace.status() == TraceStatus::Outside &&
				(args.range_start..args.range_stop).contains(&trace.len())
			{
				for point in trace.iter_mut() {
					if let Some((x, y)) = view.translate_point_to_view_coordinate(point) {
						coords.push((x as u32, y as u32));
					}
				}
			}
		}

		batch.coords.append(&mut coords);

		self.waiting_to_collect.push_back(batch)
	}

	fn collect_batch(&mut self) -> PinBatch {
		self.waiting_to_collect.pop_front().unwrap()
	}
}
