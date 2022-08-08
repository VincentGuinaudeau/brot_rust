
use crate::input::args::Args;
use crate::output::{ slate::Slate, view::View };
use crate::point_checker::Checker;

pub mod random;

pub trait PointFinder {
	fn execute(view: &View, args: &Args, checker: &mut dyn Checker) -> Slate;
}
