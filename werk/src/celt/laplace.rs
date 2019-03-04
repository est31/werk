// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use super::entcode::*;
use super::entdec::{ec_dec_update, ec_decode_bin};
use std::cmp;
use std::os::raw::*;

/// The minimum probability of an energy delta (out of 32768).
const LAPLACE_LOG_MINP: u32 = 0;
const LAPLACE_MINP: u32 = 1 << LAPLACE_LOG_MINP;
/// The minimum number of guaranteed representable energy deltas (in one
/// direction).
const LAPLACE_NMIN: u32 = 16;

// TODO one day create a laplace ctx struct that contains fs and decay,
// and convert the three functions in this file to functions over that ctx.

// When called, decay is positive and at most 11456.
fn laplace_get_freq1(fs0: c_uint, decay: c_int) -> c_uint {
	let ft = 32768 - LAPLACE_MINP * (2 * LAPLACE_NMIN) - fs0;
	(ft * ((16384 - decay) as u32)) >> 15
}

#[no_mangle]
/// Encode a value that is assumed to be the realisation of a
/// Laplace-distributed random process
///
/// * `enc` - Entropy encoder state
/// * `value` - Value to encode
/// * `fs` - Probability of 0, multiplied by 32768
/// * `decay` - Probability of the value +/- 1, multiplied by 16384
pub extern "C" fn ec_laplace_encode(
	enc: &mut ec_ctx,
	value: &mut c_int,
	mut fs: c_uint,
	decay: c_int,
) {
	let mut fl: c_uint = 0;
	let mut val: c_int = *value;
	if val != 0 {
		let s = -((val < 0) as c_int);
		val = (val + s) ^ s;
		fl = fs;
		fs = laplace_get_freq1(fs, decay);
		// Search the decaying part of the PDF.
		let mut last_i = 0;
		for i in 1..val {
			last_i = i;
			if fs == 0 {
				break;
			}
			fs *= 2;
			fl += fs + 2 * (LAPLACE_MINP as u32);
			fs = (fs * decay as u32) >> 15;
		}
		// Everything beyond that has probability LAPLACE_MINP
		if fs == 0 {
			let mut ndi_max = (32768 - fl + LAPLACE_MINP - 1) >> LAPLACE_LOG_MINP;
			ndi_max = (ndi_max - s as c_uint) >> 1;
			let di = cmp::min(val - last_i, ndi_max as i32 - 1);
			fl += (2 * di + 1 + s) as u32 * LAPLACE_MINP;
			fs = cmp::min(LAPLACE_MINP, 32768 - fl);
			*value = (last_i + di + s) ^ s;
		} else {
			fs += LAPLACE_MINP;
			fl += fs & !(s as u32);
		}
	}
	unsafe {
		ec_encode_bin(enc, fl, fl + fs, 15);
	}
}

extern "C" {
	fn ec_encode_bin(this: &mut ec_ctx, fl: c_uint, fh: c_uint, bits: c_uint);
}

#[no_mangle]
/// Decode a value that is assumed to be the realisation of a
/// Laplace-distributed random process
///
/// * `dec` - Entropy decoder state
/// * `fs` - Probability of 0, multiplied by 32768
/// * `decay` - Probability of the value +/- 1, multiplied by 16384
pub extern "C" fn ec_laplace_decode(dec: &mut ec_ctx, mut fs: c_uint, decay: c_int) -> c_int {
	let mut val: c_int = 0;
	let mut fl: c_uint = 0;
	let fm: c_uint = ec_decode_bin(dec, 15);
	if fm >= fs {
		val += 1;
		fl = fs;
		fs = laplace_get_freq1(fs, decay) + LAPLACE_MINP;
		// Search the decaying part of the PDF.
		while fs > LAPLACE_MINP && fm >= fl + 2 * fs {
			fs *= 2;
			fl += fs;
			fs = ((fs - 2 * LAPLACE_MINP) * (decay as c_uint)) >> 15;
			fs += LAPLACE_MINP;
			val += 1;
		}
		// Everything beyond that has probability LAPLACE_MINP.
		if fs <= LAPLACE_MINP {
			let di = (fm - fl) >> (LAPLACE_LOG_MINP + 1);
			val += di as c_int;
			fl += 2 * di * LAPLACE_MINP;
		}
		if fm < fl + fs {
			val = -val;
		} else {
			fl += fs;
		}
	}
	assert!(fl < 32768);
	assert!(fs > 0);
	assert!(fl <= fm);
	assert!(fm < cmp::min(fl + fs, 32768));
	ec_dec_update(dec, fl, cmp::min(fl + fs, 32768), 32768);
	val
}
