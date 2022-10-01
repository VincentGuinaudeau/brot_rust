
use crate::core::{ trace::Trace, batch::PinBatch };
use crate::input::args::Args;
use crate::output::view::View;

pub type TraceInitializer = Box<dyn FnMut(&mut Trace) -> &mut Trace + Send + Sync>;

pub mod classic;
pub mod vectorized;
pub mod threaded;

pub trait Checker {
	fn get_batch_ideal_capacity(&self) -> usize;
	fn push_batch(&mut self, view: &View, args: &Args, batch: PinBatch, trace_init: TraceInitializer);
	fn collect_batch(&mut self) -> PinBatch;
	fn done(&mut self) {}
}
