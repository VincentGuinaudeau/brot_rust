/* An implementation of the MT19937 Algorithm for the Mersenne Twister
 * by Evan Sultanik.  Based upon the pseudocode in: M. Matsumoto and
 * T. Nishimura, "Mersenne Twister: A 623-dimensionally
 * equidistributed uniform pseudorandom number generator," ACM
 * Transactions on Modeling and Computer Simulation Vol. 8, No. 1,
 * January pp.3-30 1998.
 *
 * http://www.sultanik.com/Mersenne_twister
 */

/*
 * Copied from https://github.com/ESultanik/mtwister/tree/ae3ace33db7db7b42a18e3d4eeb4b7dd3467b1ec
 * Rewritten in rust.
 */

use std::num::Wrapping;

use super::point::{ Point, FractFloat };

const UPPER_MASK:Wrapping<u32> = Wrapping(0x80000000);
const LOWER_MASK:Wrapping<u32> = Wrapping(0x7fffffff);
const TEMPERING_MASK_B:u32     = 0x9d2c5680;
const TEMPERING_MASK_C:u32     = 0xefc60000;

const STATE_VECTOR_LENGTH:usize = 624;
const STATE_VECTOR_M:usize      = 397; /* changes to STATE_VECTOR_LENGTH also require changes to this */

pub struct Rng {
	mt:    [Wrapping<u32>; STATE_VECTOR_LENGTH],
	index: usize,
}

impl Rng {
	pub fn new(seed: u32) -> Rng {
		/* set initial seeds to mt[STATE_VECTOR_LENGTH] using the generator
		 * from Line 25 of Table 1 in: Donald Knuth, "The Art of Computer
		 * Programming," Vol. 2 (2nd Ed.) pp.102.
		 */
		let mut rng = Rng {
			mt: [Wrapping(0); STATE_VECTOR_LENGTH],
			index: STATE_VECTOR_LENGTH - 1,
		};
		rng.set_rng(seed);
		rng
	}

	fn set_rng(&mut self, seed: u32) {
		self.mt[0] = Wrapping(seed & (0xffffffff as u32));
		for i in 1..self.mt.len() {
			self.mt[i] = Wrapping(6069) * self.mt[i-1];
		}
	}

	/**
	 * Generates a pseudo-randomly generated long.
	 */
	pub fn u32(&mut self) -> u32{
		let mut y;
		let mag = [0x0 as u32, 0x9908b0df as u32];

		if self.index >= STATE_VECTOR_LENGTH {
			/* generate STATE_VECTOR_LENGTH words at a time */
			if self.index >= STATE_VECTOR_LENGTH + 1 {
				self.set_rng(4357);
			}
			let mut kk = 0;
			while kk < STATE_VECTOR_LENGTH - STATE_VECTOR_M {
				y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
				self.mt[kk] = self.mt[kk + STATE_VECTOR_M]                       ^ Wrapping((y.0 >> 1) ^ mag[if y.0 % 2 == 1 { 1 } else { 0 }]);
				kk += 1;
			}
			while kk < STATE_VECTOR_LENGTH - 1 {
				y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
				self.mt[kk] = self.mt[kk + STATE_VECTOR_M - STATE_VECTOR_LENGTH] ^ Wrapping((y.0 >> 1) ^ mag[if y.0 % 2 == 1 { 1 } else { 0 }]);
				kk += 1;
			}
			y = (self.mt[STATE_VECTOR_LENGTH - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
			self.mt[STATE_VECTOR_LENGTH - 1] = self.mt[STATE_VECTOR_M - 1]       ^ Wrapping((y.0 >> 1) ^ mag[if y.0 % 2 == 1 { 1 } else { 0 }]);
			self.index = 0
		}
		let mut z: u32 = self.mt[self.index].0;
		self.index += 1;
		z ^= z >> 11;
		z ^= (z << 7) & TEMPERING_MASK_B;
		z ^= (z << 15) & TEMPERING_MASK_C;
		z ^= z >> 18;
		z
	}

	pub fn u64(&mut self) -> u64 {
		(self.u32() as u64) << 32 | self.u32() as u64
	}

	/**
	 * Generates a pseudo-randomly f32 double in the range [0..1].
	 */
	// pub fn f32(&mut self) -> f32 {
	// 	(self.u32() as f32) / ((0xffffffff as u32) as f32)
	// }

	/**
	 * Generates a pseudo-randomly f64 double in the range [0..1].
	 */
	pub fn f64(&mut self) -> f64 {
		(self.u64() as f64) / ((0xffffffffffffffff as u64) as f64)
	}

	pub fn point(&mut self) -> Point {
		Point::new( self.f64() * 4. - 2., self.f64() * 4. - 2. )
	}
}
