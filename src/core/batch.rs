
use std::boxed::Box;
use std::pin::Pin;

use super::{ trace::Trace, point::Point };

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

	pub fn reset<F, Arg>(&mut self, number_of_elem: usize, f: F, arg: &mut Arg) where F: Fn(&mut Arg) -> Point {

		for index in 0..number_of_elem {
			let maybe_elem = self.traces.get_mut(index);
			if let Some( trace ) = maybe_elem {
				trace.reset(f(arg));
			}
			else {
				let mut new_trace = Trace::new(self.trace_length);
				new_trace.reset(f(arg));
				self.traces.push(new_trace);
			}
		}
		self.traces.truncate(number_of_elem);
		self.coords.clear();
	}
}
