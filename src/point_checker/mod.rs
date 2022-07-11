
use crate::core::trace::Trace;

pub mod classic;

pub trait Checker {
	fn push_batch(&mut self, batch: Vec< Trace >);
	fn collect_batch(&mut self) -> Option< Vec< Trace > >;
}
