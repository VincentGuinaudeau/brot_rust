
use crate::core::batch::PinBatch;
use crate::input::search_parameters::SearchParameters;
use crate::output::view::View;

pub mod classic;
pub mod threaded;

pub trait Checker {
	fn get_batch_ideal_capacity(&self) -> usize;
	fn push_batch(&mut self, view: &View, search_param: &SearchParameters, batch: PinBatch);
	fn collect_batch(&mut self) -> PinBatch;
	fn done(&mut self) {}
}
