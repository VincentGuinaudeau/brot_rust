
use std::boxed::Box;
use std::pin::Pin;

pub struct Batch {
	pub trace_length: usize,
	pub size: usize,
	pub coords: Vec< (u32, u32, u16) >,
}

pub type PinBatch = Pin< Box < Batch > >;

impl Batch {
	pub fn new(trace_length: usize, capacity: usize) -> PinBatch {
		let batch = Batch {
			trace_length,
			size: capacity,
			coords: Vec::with_capacity(capacity / 2 * trace_length),
		};

		Box::pin(batch)
	}

	pub fn reset(&mut self) {
		self.coords.clear();
	}
}
