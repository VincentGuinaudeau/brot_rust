
// use clap::Parser;

mod core;
use crate::core::point::Point;

mod input;
use crate::input::search_parameters::SearchParameters;

mod output;
use crate::output::view::View;

mod point_checker;
use crate::point_checker::*;

mod points_finder;
use crate::points_finder::*;

fn main() {
	println!("Hello, world!");

	let search_param = SearchParameters {
		lower_bound: 10,
		upper_bound: 50,
	};

	let view = View::new(4_000, 4_000, Point::new_null(), 2.);

	let mut checker = threaded::ThreadedChecker::new();

	let slate = random::RandomPointFinder::execute(&view, &search_param, &mut checker);

	slate.to_png("./out.png");
}
