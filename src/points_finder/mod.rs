
use crate::input::args::{ Args, Algorithm };
use crate::output::{ slate::Slate, view::View };
use crate::point_checker::Checker;

pub mod random;
use random::RandomPointFinder;

pub mod scan;
use scan::ScanPointFinder;

pub trait PointFinder {
	fn execute(view: &View, args: &Args, checker: &mut dyn Checker) -> Slate;
}

pub fn launch_finder(view: &View, args: &Args, checker: &mut dyn Checker) -> Slate {
	match args.algorithm {
		Algorithm::Random => RandomPointFinder::execute(view, args, checker),
		Algorithm::Scan   => ScanPointFinder::execute(view, args, checker),
	}
}
