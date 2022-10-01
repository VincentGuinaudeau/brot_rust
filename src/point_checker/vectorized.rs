
use std::simd::*;
use std::collections::vec_deque::VecDeque;
use super::{ Checker, View, Args, TraceInitializer };
use crate::core::{ trace::{ Trace, TraceStatus }, batch::PinBatch, point::Point };

pub struct VectorizedChecker {
	waiting_to_collect: VecDeque< PinBatch >,
}

impl VectorizedChecker {
	pub fn new() -> VectorizedChecker {
		VectorizedChecker {
			waiting_to_collect: VecDeque::new(),
		}
	}
}

impl Checker for VectorizedChecker {
	fn get_batch_ideal_capacity(&self) -> usize { 1 }

	fn push_batch(&mut self, view: &View, args: &Args, mut batch: PinBatch, mut trace_init: TraceInitializer) {

		let mut drum_r         = f64x8::splat(0.);
		let mut drum_i         = f64x8::splat(0.);
		let mut drum_squared_r = f64x8::splat(0.);
		let mut drum_squared_i = f64x8::splat(0.);
		let mut drum_origin_r  = f64x8::splat(0.);
		let mut drum_origin_i  = f64x8::splat(0.);
		let mut drum_tmp;

		let mut loop_count = u64x8::splat(0);
		let loop_inc       = u64x8::splat(1);
		let loop_max       = u64x8::splat(args.range_stop as u64);

		let drum_two  = f64x8::splat(2.);
		let drum_four = f64x8::splat(4.);

		let mut loaded_lanes = mask64x8::splat(false);
		let mut mask_tmp;
		let mut mask_done;

		let mut trace_iter = batch.traces.iter_mut();

		let mut current_traces: [Option< &mut Trace >; 8] = [ None, None, None, None, None, None, None, None ];

		let mut coords:Vec< (u32, u32) > = vec![];

		// load up the drum
		for i in 0..8 {
			if let Some( trace ) = trace_iter.next() {
				let trace = trace_init(trace);
				loaded_lanes.set(i, true);
				let origin = trace.origin();
				drum_origin_r[i] = origin.r();
				drum_origin_i[i] = origin.i();
				loaded_lanes.set(i, true);
				current_traces[i] = Some( trace );
			}
			else {
				break;
			}
		}

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
					current_traces[i].as_mut().unwrap().extend(Point::new( drum_r[i], drum_i[i]));
				}
			}

			// If a trace is done, unload it, maybe write the coords, and maybe load another one
			mask_done = loop_count.simd_ge(loop_max) & loaded_lanes;
			mask_tmp  = (drum_squared_r + drum_squared_i).simd_gt(drum_four);
			mask_done = mask_done | mask_tmp;
			if mask_done.any() {
				for i in 0..8 {
					if mask_done.test(i) {
						let trace = current_traces[i].as_mut().unwrap();
						if 
							trace.status() == TraceStatus::Outside &&
							(args.range_start..args.range_stop).contains(&trace.len())
						{
							for point in trace.iter_mut() {
								if let Some((x, y)) = view.translate_point_to_view_coordinate(point) {
									coords.push((x as u32, y as u32));
								}
							}
						}

						let next_trace = trace_iter.next();
						if let Some(trace) = next_trace {
							let trace = trace_init(trace);
							let origin = trace.origin();
							drum_origin_r[i] = origin.r();
							drum_origin_i[i] = origin.i();
							drum_r[i] = 0.;
							drum_i[i] = 0.;
							drum_squared_r[i] = 0.;
							drum_squared_i[i] = 0.;
							current_traces[i] = Some( trace );
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

	fn collect_batch(&mut self) -> PinBatch {
		self.waiting_to_collect.pop_front().unwrap()
	}
}
