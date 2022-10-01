
use std::boxed::Box;
use std::pin::Pin;

use super::trace::Trace;

pub struct Batch {
	trace_length: usize,
	pub traces: Vec< Trace >,
	pub coords: Vec< (u32, u32) >,
}

pub type PinBatch = Pin< Box < Batch > >; 

impl Batch {
	pub fn new(trace_length: usize, capacity: usize) -> PinBatch {
		let batch = Batch {
			trace_length,
			traces: Vec::with_capacity(capacity),
			coords: Vec::with_capacity(capacity / 2 * trace_length),
		};

		Box::pin(batch)
	}

	pub fn reset(&mut self, number_of_elem: usize) {

		if self.traces.len() > number_of_elem {
			self.traces.truncate(number_of_elem);
		}
		else if self.traces.len() < number_of_elem {
			while self.traces.len() < number_of_elem {
				self.traces.push(Trace::new(self.trace_length));
			}
		}
		self.coords.clear();
	}
}
