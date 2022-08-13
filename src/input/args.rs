
use std::str::FromStr;

use std::fmt::Display;

use clap::{ Parser, ArgEnum };

use crate::core::point::{ Point, FractFloat };

#[derive(Copy, Clone, Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

	// --About the image

	/// The type of info to render
	#[clap( long = "style", short = 'S', arg_enum, default_value_t = RenderStyle::Buddhabrot )]
	pub render_type: RenderStyle,

	/// width of the image
	#[clap( long = "width", short = 'w', default_value_t = 1024 )]
	pub view_width: i32,

	/// height of the image
	#[clap( long = "height", short = 'h', default_value_t = 1024 )]
	pub view_height: i32,

	/// where to center the render
	#[clap( long = "offset", short = 'o', default_value_t = Point::new_null() )]
	pub view_offset: Point,

	/// how zomed in the render is. Default is 2, lower values mean more zoomed in
	#[clap( long = "zoom", short = 'z', default_value_t = 2. )]
	pub view_zoom: FractFloat,

	/// Gamma correction : auto, off, or a value between 0 and 1
	#[clap( long = "gamma", short = 'g', default_value_t = GammaSetting::Auto )]
	pub gamma: GammaSetting,

	// --About the rendering

	/// The algorithm used to find relevant points
	#[clap( long = "algorithm", short = 'a', arg_enum, default_value_t = Algorithm::Random )]
	pub algorithm: Algorithm,

	/// number of thread to launch, 0 for auto
	#[clap( long = "threads", short = 'T', default_value_t = 0 )]
	pub thread_num: usize,

	/// for random algorithm, the number of points to iterate over
	#[clap( long = "sample", short = 's', default_value_t = 0 )]
	pub point_sample: usize,

	/// for random algorithm, the number of points to find
	#[clap( long = "target", short = 't', default_value_t = 200_000 )]
	pub point_target: usize,

	/// for random algorithm, the number of points to find
	#[clap( long = "batch", short = 'b', default_value_t = 100 )]
	pub batch_size: usize,

	/// The start of the range of points to find
	#[clap( long = "minimum", short = 'm', default_value_t = 10 )]
	pub range_start: usize,

	/// The end of the range of points to find
	#[clap( long = "maximum", short = 'M', default_value_t = 20 )]
	pub range_stop: usize,

	// --About the fractal
	// -F mandelbrot|julia : (Fractal) The kind of function to iterate on. Default is mandelbrot.
	// -p DECIMAL : (Power) The value for the power parameter. Default is 2.
	// -f COMPLEX : (Factor) The value for the factor parameter. Default is 1+0i.
	// -j COMPLEX : (Julia) The additionnal parameter needed for the Julia function. Default is 0+0i.
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum RenderStyle {
	Binary,
	Layered,
	Buddhabrot,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum Algorithm {
	Random,
	Scan,
	Tree,
	MetroHast,
}

#[derive( Copy, Clone, PartialEq, Debug )]
pub enum GammaSetting {
	Off,
	Auto,
	Value( f32 ),
}

impl FromStr for GammaSetting {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"off"  => Ok( GammaSetting::Off ),
			"auto" => Ok( GammaSetting::Auto ),
			_      => {
				let value = s.parse::<f32>().map_err(|_err| "Invalid value".to_string())?;
				if value < 0. || value > 1. {
					Err("Gamma value must be comprised between zero and one".to_string())
				}
				else {
					Ok( GammaSetting::Value( value ) )
				}
			}
		}
	}
}

impl Display for GammaSetting {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			GammaSetting::Off            => "off".to_string(),
			GammaSetting::Auto           => "auto".to_string(),
			GammaSetting::Value( value ) => value.to_string(),
		})
	}
}
