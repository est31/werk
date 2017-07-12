// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use std::os::raw::*;
use super::mathops::ilog;

pub type ec_window = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ec_ctx {
    pub buf :*mut c_uchar,
    pub storage :u32,
    pub end_offs :u32,
    pub end_window :ec_window,
    pub nend_bits :c_int,
    pub nbits_total :c_int,
    pub offs :u32,
    pub rng :u32,
    pub val :u32,
    pub ext :u32,
    pub rem :c_int,
    pub error :c_int,
}

/// The resolution of fractional-precision bit usage measurements, i.e.,
/// 3 => 1/8th bits.
pub const BITRES :usize = 3;

#[no_mangle]
/**
Returns the number of bits "used" by the encoded or decoded symbols so far.
This same number can be computed in either the encoder or the decoder, and is
suitable for making coding decisions.
The number of bits scaled by 2**BITRES.
This will always be slightly larger than the exact value (e.g., all
rounding error is in the positive direction).
*/
pub extern fn ec_tell_frac(this :&mut ec_ctx) -> u32 {
	// This is a faster version of ec_tell_frac() that takes advantage
	// of the low (1/8 bit) resolution to use just a linear function
	// followed by a lookup to determine the exact transition thresholds.
	const CORRECTION :[u32; 8] = [
		35733, 38967, 42495, 46340,
		50535, 55109, 60097, 65535
	];
	let nbits = (this.nbits_total as u32) << BITRES;
	let mut l = ilog(this.rng);
	let r = this.rng >> (l - 16);
	let mut b :u32 = (r >> 12) - 8;
	b += (r > CORRECTION[b as usize]) as u32;
	l = (l << 3) + b;
	return nbits - l;
}

pub extern fn ec_tell_frac_slow(this :&mut ec_ctx) -> u32 {
	// To handle the non-integral number of bits still left in the encoder/decoder
	// state, we compute the worst-case number of bits of val that must be
	// encoded to ensure that the value is inside the range for any possible
	// subsequent bits.
	// The computation here is independent of val itself (the decoder does not
	// even track that value), even though the real number of bits used after
	// ec_enc_done() may be 1 smaller if rng is a power of two and the
	// corresponding trailing bits of val are all zeros.
	// If we did try to track that special case, then coding a value with a
	// probability of 1/(1<<n) might sometimes appear to use more than n bits.
	// This may help explain the surprising result that a newly initialized
	// encoder or decoder claims to have used 1 bit.
	let nbits = (this.nbits_total as u32) << BITRES;
	let mut l = ilog(this.rng);
	let mut r = this.rng >> (l - 16);
	for _ in 0 .. BITRES {
		r = r * r >> 15;
		let b = r >> 16;
		l = (l << 1) | b;
		r >>= b;
	}
	return nbits - l;
}
