
use std::simd::*;
use std::collections::vec_deque::VecDeque;
use std::cmp::min;
use super::{
	Checker,
	View,
	Args,
	TraceInitializer,
};
use crate::point_renderer::{ select_point_renderer, PointRenderer };
use crate::core::{ trace::Trace, batch::PinBatch, point::{ Point, FractFloat } };

type FloatType = FractFloat;
const LANES: usize = 8;

type FloatVec = Simd<FloatType, LANES>;
type IntVec   = Simd<u64,        LANES>;

pub struct VectorizedChecker {
	point_renderer: Box<dyn PointRenderer>,
	waiting_to_collect: VecDeque< PinBatch >,
}

impl VectorizedChecker {
	pub fn new(args: &Args) -> VectorizedChecker {
		VectorizedChecker {
			point_renderer: select_point_renderer(args),
			waiting_to_collect: VecDeque::new(),
		}
	}
}

impl Checker for VectorizedChecker {
	fn get_batch_ideal_capacity(&self) -> usize { 1 }

	fn push_batch(&mut self, view: &View, args: &Args, mut batch: PinBatch, mut trace_init: TraceInitializer) {

		let mut drum_r         = FloatVec::splat(0.);
		let mut drum_i         = FloatVec::splat(0.);
		let mut drum_squared_r = FloatVec::splat(0.);
		let mut drum_squared_i = FloatVec::splat(0.);
		let mut drum_origin_r  = FloatVec::splat(0.);
		let mut drum_origin_i  = FloatVec::splat(0.);
		let mut drum_tmp;

		let mut loop_count = IntVec::splat(0);
		let loop_inc       = IntVec::splat(1);
		let loop_max       = IntVec::splat(args.range_stop as u64);

		let drum_two  = FloatVec::splat(2.);
		let drum_four = FloatVec::splat(4.);

		let mut loaded_lanes = mask64x8::splat(false);
		let mut mask_tmp;
		let mut mask_done;

		let mut current_traces: [Trace; LANES] = [
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
			Trace::new(args.range_stop),
		];

		let mut coords:Vec< (u32, u32, u16) > = vec![];

		// load up the drum
		for i in 0..(min(LANES, batch.size)) {
			trace_init(&mut current_traces[i]);

			let origin = current_traces[i].origin();
			drum_origin_r[i] = origin.r();
			drum_origin_i[i] = origin.i();
			loaded_lanes.set(i, true);
		}

		let mut trace_initialized = 8;

		// while there is stuff in the drum
		while loaded_lanes.any() {

			// Do a round
			drum_tmp = drum_squared_r - drum_squared_i + drum_origin_r;
			drum_i = drum_two * drum_r * drum_i + drum_origin_i;
			drum_r = drum_tmp;

			drum_squared_r = drum_r * drum_r;
			drum_squared_i = drum_i * drum_i;

			loop_count = loop_count + loop_inc;

			// inscribe it in the traces and record it for the coords
			for i in 0..8 {
				if loaded_lanes.test(i) {
					current_traces[i].extend(Point::new( drum_r[i], drum_i[i]));
				}
			}

			// If a trace is done, unload it, maybe write the coords, and maybe load another one
			mask_done = loop_count.simd_ge(loop_max) & loaded_lanes;
			mask_tmp  = (drum_squared_r + drum_squared_i).simd_gt(drum_four);
			mask_done = mask_done | mask_tmp;
			if mask_done.any() {
				for i in 0..LANES {
					if mask_done.test(i) {

						self.point_renderer.as_ref().render(view, &mut coords, &current_traces[i]);

						if trace_initialized < batch.trace_length
						{
							trace_init(&mut current_traces[i]);
							trace_initialized += 1;

							let origin = current_traces[i].origin();
							drum_origin_r[i] = origin.r();
							drum_origin_i[i] = origin.i();
							drum_r[i] = 0.;
							drum_i[i] = 0.;
							drum_squared_r[i] = 0.;
							drum_squared_i[i] = 0.;
							loop_count[i] = 0;
						}
						else {
							loaded_lanes.set(i, false);
						}
					}
				}
			}
		}

		batch.coords.append(&mut coords);

		self.waiting_to_collect.push_back(batch)
	}

	fn collect_batch(&mut self) -> Option< PinBatch > {
		self.waiting_to_collect.pop_front()
	}
}
