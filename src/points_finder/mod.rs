
use crate::input::search_parameters::SearchParameters;
use crate::output::{ slate::Slate, view::View };
use crate::point_checker::Checker;

pub mod random;

pub trait PointFinder {
	fn execute(view: &View, search_param: &SearchParameters, checker: &mut dyn Checker) -> Slate;
}
