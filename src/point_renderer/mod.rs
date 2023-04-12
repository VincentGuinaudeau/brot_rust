
use crate::{
	core::trace::{ Trace, TraceStatus },
	input::args::{ Args, RenderType },
	output::view::View,
};

mod binary;
use binary::BinaryPointRenderer;

mod layered;
use layered::LayeredPointRenderer;

mod buddhabrot;
use buddhabrot::BuddhebrotPointRenderer;

pub trait PointRenderer {
	fn render(&self, view: &View, coords: &mut Vec< (u32, u32, u16) >, trace: &Trace) -> ();
}

pub fn select_point_renderer(args: &Args) -> Box< dyn PointRenderer > {
	match args.render_type {
		RenderType::Binary     => Box::new(BinaryPointRenderer::new()),
		RenderType::Layered    => Box::new(LayeredPointRenderer::new()),
		RenderType::Buddhabrot => Box::new(BuddhebrotPointRenderer::new(args)),
	}
}
