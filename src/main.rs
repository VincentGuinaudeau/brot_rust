#![feature(portable_simd)]

use clap::Parser;

mod core;

mod input;
use crate::input::args::Args;

mod output;
use crate::output::view::View;

mod point_renderer;

mod point_checker;
use crate::point_checker::*;

mod points_finder;
use crate::points_finder::*;

fn main() {

	let args = Args::parse();

	println!("{:?}", args);

	let view = View::new(args.view_width, args.view_height, args.view_offset, args.view_zoom);

	// let mut checker = classic::ClassicChecker::new();
	let mut checker = vectorized::VectorizedChecker::new(&args);
	// let mut checker = threaded::ThreadedChecker::new(args);

	let slate = launch_finder(&view, &args, &mut checker);

	slate.to_png("./out.png");
}
