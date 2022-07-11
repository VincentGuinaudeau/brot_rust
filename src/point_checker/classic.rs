
use super::Checker;

use std::collections::vec_deque::VecDeque;
use crate::core::trace::{ Trace, TraceStatus };

pub struct ClassicChecker {
	waiting_to_collect: VecDeque< Vec< Trace > >,
}

impl ClassicChecker {
	pub fn new() -> ClassicChecker {
		ClassicChecker {
			waiting_to_collect: VecDeque::new(),
		}
	}
}

impl Checker for ClassicChecker {

	fn push_batch(&mut self, mut batch: Vec< Trace >) {
		for trace in &mut batch {
			while trace.status() == TraceStatus::NotDone {
				let new_point = trace.tail().squared() + trace.origin();
				trace.extend(new_point);
			}
		}
		self.waiting_to_collect.push_back(batch)
	}

	fn collect_batch(&mut self) -> Option< Vec< Trace > > {
		self.waiting_to_collect.pop_front()
	}
}
