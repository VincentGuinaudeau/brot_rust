
use std::simd::*;
use std::collections::vec_deque::VecDeque;
use super::{ Checker, View, Args };
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

	fn push_batch(&mut self, view: &View, args: &Args, mut batch: PinBatch) {

		// state for computing the points

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

		let const_two  = f64x8::splat(2.);
		let const_four = f64x8::splat(4.);

		let mut loaded_lanes = mask64x8::splat(false);
		let mut mask_tmp;
		let mut mask_done;

		let mut trace_iter = batch.traces.iter_mut();

		let mut current_traces: [Option< &mut Trace >; 8] = [ None, None, None, None, None, None, None, None ];

		// state for computing the coordinates

		let mut count = 0;

		let step        = f64x8::splat(view.step());
		let position_r  = f64x8::splat(view.position().r());
		let position_i  = f64x8::splat(view.position().i());
		let half_x_size = i32x8::splat(view.x_size() / 2);
		let half_y_size = i32x8::splat(view.y_size() / 2);
		let zero        = i32x8::splat(0);
		let x_size      = i32x8::splat(view.x_size());
		let y_size      = i32x8::splat(view.y_size());

		let mut r = f64x8::splat(0.);
		let mut i = f64x8::splat(0.);

		let mut coords:Vec< (u32, u32) > = vec![];

		// load up the drum
		for i in 0..8 {
			if let Some( trace ) = trace_iter.next() {
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
			drum_i = const_two * drum_r * drum_i + drum_origin_i;
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

			// check some conditions on all the loaded traces
			mask_done = loop_count.lanes_ge(loop_max) & loaded_lanes;
			mask_tmp  = (drum_squared_r + drum_squared_i).lanes_gt(const_four);
			mask_done = mask_done | mask_tmp;

			// If a trace is done, unload it, maybe write the coords, and maybe load another one
			if mask_done.any() {
				for index in 0..8 {
					if mask_done.test(index) {
						let trace = current_traces[index].as_mut().unwrap();
						if 
							trace.status() == TraceStatus::Outside &&
							(args.range_start..args.range_stop).contains(&trace.len())
						{
							for point in trace.iter() {
								r[count] = point.r();
								i[count] = point.i();
								count += 1;

								if count == 8 {

									let x:i32x8 = ((r - position_r) / step).floor().cast() + half_x_size;
									let y:i32x8 = ((i - position_i) / step).floor().cast() + half_y_size;

									let accepted = x.lanes_ge(zero) & x.lanes_lt(x_size) & y.lanes_ge(zero) & y.lanes_lt(y_size);

									for i in 0..8 {
										if accepted.test(i) {
											coords.push((x[i] as u32, y[i] as u32));
										}
									}
									count = 0;
								}
							}
						}

						let next_trace = trace_iter.next();
						if let Some(trace) = next_trace {
							let origin = trace.origin();
							drum_origin_r[index] = origin.r();
							drum_origin_i[index] = origin.i();
							drum_r[index] = 0.;
							drum_i[index] = 0.;
							drum_squared_r[index] = 0.;
							drum_squared_i[index] = 0.;
							current_traces[index] = Some( trace );
							loop_count[index] = 0;
						}
						else {
							loaded_lanes.set(index, false);
						}
					}
				}
			}
		}

		if count != 0 {
			let x:i32x8 = ((r - position_r) / step).floor().cast() + half_x_size;
			let y:i32x8 = ((i - position_i) / step).floor().cast() + half_y_size;

			let accepted = x.lanes_ge(zero) & x.lanes_lt(x_size) & y.lanes_ge(zero) & y.lanes_lt(y_size);

			for i in 0..count {
				if accepted.test(i) {
					coords.push((x[i] as u32, y[i] as u32));
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
