// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

/*!
Range decoding functionality

See [section 4.1 of RFC 6716](https://tools.ietf.org/html/rfc6716#section-4.1).
*/

pub trait RangeContext {
	type out;

	fn get_ft(&self) -> u16;
	fn get_k_fl_fh(&self, fs :u16) -> (Self::out, u16, u16);
}

pub struct RangeDecoder<'a> {
	data :&'a[u8],
	idx :usize,
	val :u32,
	rng :u32,
	bit_remain : u8,
}

impl<'a> RangeDecoder<'a> {
	pub fn new(data :&'a [u8]) -> Self {
		let b0 = data.get(0).cloned().unwrap_or(0);
		return RangeDecoder {
			data : data,
			idx : 1,
			val : (127 - (b0 >> 1) as u32),
			rng : 128,
			bit_remain : (b0 & 1),
		};
	}
	pub fn decode_symbol<C :RangeContext>(&mut self, context :&C) -> C::out {
		let ft = context.get_ft();
		let rng_d_ft = self.rng / ft as u32;
		let fs :u16 = ft.checked_sub(
			(self.val /  rng_d_ft) as u16 + 1
			).unwrap_or(0);

		let (k, fl, fh) = context.get_k_fl_fh(fs);

		// State updates
		self.val = self.val - rng_d_ft * (ft - fh) as u32;
		if fl > 0 {
			self.rng = rng_d_ft * (fh - fl) as u32;
		} else {
			self.rng = self.rng - rng_d_ft * (ft - fh) as u32;
		}

		// Renormalisation (4.1.2.1.)
		while self.rng <= (1 << 23) {
			self.rng = self.rng << 8;
			let next_byte = self.data.get(self.idx).cloned().unwrap_or(0);
			self.idx += 1;
			let new_bit_remain = next_byte & 1;
			let sym = (self.bit_remain << 7) | (next_byte >> 1);
			self.bit_remain = new_bit_remain;
			self.val = self.val.wrapping_shl(8) + (255 - sym) as u32;
		}
		return k;
	}
	// TODO Maybe implement alternate decoding methods
	// for speedup (4.1.3.)

	// TODO raw bit decoding (4.1.4.)

	// TODO uniformly distributed integer decoding (4.1.5.)

	// TODO Current bit usage info (4.1.6.)
}
