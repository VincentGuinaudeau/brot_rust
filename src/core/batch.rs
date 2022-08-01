
use std::boxed::Box;
use std::pin::Pin;

use super::{ trace::Trace, point::Point };

const BATCH_SIZE:usize = 1_000;

pub struct Batch {
	pub traces: Vec< Trace >,
	pub coords: Vec< (u32, u32) >,
}

pub type PinBatch = Pin< Box < Batch > >; 

impl Batch {
	pub fn new(trace_length: usize) -> PinBatch {
		let mut batch = Batch {
			traces: Vec::with_capacity(BATCH_SIZE),
			coords: Vec::with_capacity(BATCH_SIZE / 2 * trace_length),
		};

		for _i in 0..BATCH_SIZE {
			batch.traces.push(Trace::new(trace_length));
		}

		Box::pin(batch)
	}

	pub fn reset<F, Arg>(&mut self, f: F, arg: &mut Arg) where F: Fn(&mut Arg) -> Point {
		for trace in self.traces.iter_mut() {
			trace.reset(f(arg));
		}
		self.coords.clear();
	}
}
