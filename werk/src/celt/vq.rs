// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use super::arch::{celt_norm, v16, v32, EPSILON};
use super::cwrs::{decode_pulses, encode_pulses};
use super::entcode::{celt_udiv, ec_ctx};
use super::mathops::fast_atan2;
use super::pitch::inner_prod_rs;
use std::f32::consts::FRAC_2_PI;
use std::os::raw::*;
use std::slice;

use super::bands::SPREAD_NONE;

unsafe fn exp_rotation_1(x: *mut celt_norm, off: usize, len: usize, stride: usize, c: v16, s: v16) {
	let ms = neg16!(s);
	let off = off as isize;
	let stride = stride as isize;
	let len = len as isize;
	let mut xptr = x.offset(off);

	macro_rules! epmm {
		($a:ident, $b:ident, $s:ident) => {
			extract16!(pshr32!(mac16_16!(mult16_16!(c, $a), $s, $b), 15))
		};
	}
	for _ in 0..len - stride {
		let x1 = *xptr;
		let x2 = *xptr.offset(stride);
		*xptr.offset(stride) = epmm!(x2, x1, s);
		*xptr = epmm!(x1, x2, ms);
		xptr = xptr.offset(1);
	}
	// Sadly, we can't use Rust slices in this function yet
	// because we are writing to negative indices of X here.
	// We'd have to change it somewhere higher up in the chain.
	// Maybe it would work if we changed it in celt/bands.c...
	xptr = x.offset(off + len - 2 * stride - 1);
	for _ in 0..len - 2 * stride {
		let x1 = *xptr;
		let x2 = *xptr.offset(stride);
		*xptr.offset(stride) = epmm!(x2, x1, s);
		*xptr = epmm!(x1, x2, ms);
		xptr = xptr.offset(-1);
	}
}

#[no_mangle]
pub unsafe extern "C" fn exp_rotation(
	x: *mut celt_norm,
	len: c_int,
	dir: c_int,
	stride: c_int,
	k: c_int,
	spread: c_int,
) {
	const SPREAD_FACTOR: [c_int; 3] = [15, 10, 5];
	if 2 * k >= len || spread == SPREAD_NONE {
		return;
	}
	let factor = SPREAD_FACTOR[spread as usize - 1];
	let gain = div!(
		mult16_16!(1.0, len as v16) as v32,
		(len + factor * k) as v32
	);
	let theta = half16!(mul16_16_q15!(gain, gain));
	let c = cos_norm!(extend32!(theta));
	let s = cos_norm!(extend32!(sub16!(1.0, theta))); // sin(theta)

	let mut stride2 = 0;

	if len >= 8 * stride {
		stride2 = 1;
		// This is a simple (equivalent) way of obtaining sqrt(len/stride)
		// with rounding.
		// We increment as long as (stride2+0.5)^2 < len/stride
		while (stride2 * stride2 + stride2) * stride + (stride >> 2) < len {
			stride2 += 1;
		}
	}
	// NOTE: As a minor optimization, we could be passing around log2(B),
	// not B, for both this and for extract_collapse_mask()
	let len = celt_udiv(len as c_uint, stride as c_uint) as usize;
	for i in 0..stride as usize {
		if dir < 0 {
			if stride2 > 0 {
				exp_rotation_1(x, i * len, len, stride2 as usize, s, c);
			}
			exp_rotation_1(x, i * len, len, 1, c, s);
		} else {
			exp_rotation_1(x, i * len, len, 1, c, -s);
			if stride2 > 0 {
				exp_rotation_1(x, i * len, len, stride2 as usize, s, -c);
			}
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn op_pvq_search_c(
	x: *mut celt_norm,
	iy: *mut c_int,
	k: c_int,
	ni: c_int,
	_arch: c_int,
) -> v16 {
	let n = ni as usize;
	let x = slice::from_raw_parts_mut(x, n);
	let iy = slice::from_raw_parts_mut(iy, n);
	let mut y = Vec::with_capacity(n);
	// Get rid of the sign
	let mut signx = Vec::with_capacity(n);
	for j in 0..n {
		signx.push(x[j] < 0.0);
		// OPT: Make sure the compiler doesn't use a branch on abs16!()
		x[j] = abs16!(x[j]);
		iy[j] = 0;
		y.push(0.0);
	}
	let mut xy = 0.0;
	let mut yy = 0.0;
	let mut pulses_left = k;
	// Do a pre-search by projecting on the pyramid
	if k > (ni >> 1) {
		let mut sum = x.iter().sum();
		// Prevents infinities and NaNs from causing too many pulses
		// to be allocated. 64 is an approximation of infinity here.
		if !(sum > EPSILON && sum < 64.0) {
			x[0] = qconst16!(1.0, 14);
			for j in x.iter_mut().take(n).skip(1) {
				*j = 0.0;
			}
			sum = qconst16!(1.0, 14);
		}
		// Using K+e with e < 1 guarantees we cannot get more than K pulses.
		let rcp = extract16!(mul16_32_q16!(k as v16 + 0.8, rcp!(sum)));
		for j in 0..n {
			iy[j] = (rcp * x[j]).floor() as c_int;
			y[j] = iy[j] as celt_norm;
			yy = mac16_16!(yy, y[j], y[j]);
			xy = mac16_16!(xy, x[j], y[j]);
			y[j] *= 2.0;
			pulses_left -= iy[j];
		}
	}
	assert!(
		pulses_left >= 0,
		"Allocated too many pulses in the quick pass"
	);
	// This should never happen, but in case it does (e.g. on silence)
	// we fill the first bin with pulses.
	if pulses_left > ni + 3 {
		let tmp = pulses_left;
		yy = mac16_16!(yy, tmp, tmp);
		yy = mac16_16!(yy, tmp, y[0]);
		iy[0] += pulses_left;
		pulses_left = 0;
	}
	for _ in 0..pulses_left {
		// The squared magnitude term gets added anyway, so we might as well
		// add it outside the loop
		yy += 1.0;

		// Calculations for position 0 are out of the loop, in part to
		// reduce mispredicted branches (since the if condition is usually
		// false) in the loop.
		// Temporary sums of the new pulse(s).
		let mut rxy = extract16!(shr32!(add32!(xy, extend32!(x[0])), rshift));
		// We're multiplying y[j] by two so we don't have to do it here
		let mut ryy = add16!(yy, y[0]);
		// Approximate score: we maximize rxy/sqrt(ryy) (we're guaranteed that
		// Rxy is positive because the sign is pre-computed)
		rxy = mul16_16_q15!(rxy, rxy);
		let mut best_id = 0;
		let mut best_den = ryy;
		let mut best_num = rxy;
		for j in 1..n {
			// Temporary sums of the new pulse(s)
			rxy = extract16!(shr32!(add32!(xy, extend32!(x[j])), rshift));
			// We're multiplying y[j] by two so we don't have to do it here
			ryy = add16!(yy, y[j]);

			// Approximate score: we maximize rxy/sqrt(ryy) (we're guaranteed
			// that Rxy is positive because the sign is pre-computed)
			rxy = mul16_16_q15!(rxy, rxy);
			// The idea is to check for num/den >= best_num/best_den, but that
			// way we can do it without any division
			// OPT: It's not clear whether a cmov is faster than a branch here
			// since the condition is more often false than true and using
			// a cmov introduces data dependencies across iterations. The optimal
			// choice may be architecture-dependent.
			if unlikely!(mult16_16!(best_den, rxy) > mult16_16!(ryy, best_num)) {
				best_den = ryy;
				best_num = rxy;
				best_id = j;
			}
		}
		// Updating the sums of the new pulse(s)
		xy += x[best_id];
		// We're multiplying y[j] by two so we don't have to do it here
		yy += y[best_id];

		// Only now that we've made the final choice, update y/iy
		// Multiplying y[j] by 2 so we don't have to do it everywhere else
		y[best_id] += 2.0;
		iy[best_id] += 1;
	}
	// Put the original sign back
	for j in 0..n {
		iy[j] = if signx[j] { -iy[j] } else { iy[j] };
	}
	yy
}

/// Takes the pitch vector and the decoded residual vector,
/// computes the gain that will give ||p+g*y|| = 1 and mixes
/// the residual with the pitch.
fn normalize_residual(iy: &[c_int], x: *mut celt_norm, n: usize, ryy: v32, gain: v16) {
	let t = vshr32!(ryy, 2 * (k - 7));
	let g = mul16_16_p15!(rsqrt_norm!(t), gain);
	let x = unsafe { slice::from_raw_parts_mut(x, n) };

	for i in 0..n {
		x[i] = extract16!(pshr32!(mult16_16!(g, iy[i]), k + 1));
	}
}

fn extract_collapse_mask(iy: &[c_int], n: usize, b: usize) -> c_uint {
	if b <= 1 {
		return 1;
	}
	let mut collapse_mask = 0;
	let n0 = celt_udiv(n as c_uint, b as c_uint) as usize;
	for i in 0..b {
		let mut tmp = 0;
		for j in 0..n0 {
			tmp |= iy[i * n0 + j];
		}
		collapse_mask |= ((tmp != 0) as c_uint) << i;
	}
	collapse_mask
}

#[no_mangle]
/**
Algebraic pulse-vector quantiser. The signal x is replaced by the sum of
the pitch and a combination of pulses such that its norm is still equal
to 1. This is the function that will typically require the most CPU.

* `x` : Residual signal to quantise/encode (returns quantised version)
* `n` : Number of samples to encode
* `k` : Number of pulses to use
* `enc` : Entropy encoder

Returns a mask indicating which blocks in the band received pulses.
*/
pub unsafe extern "C" fn alg_quant(
	x: *mut celt_norm,
	n: c_int,
	k: c_int,
	spread: c_int,
	b: c_int,
	enc: &mut ec_ctx,
	gain: v16,
	resynth: c_int,
	arch: c_int,
) -> c_uint {
	assert!(k > 0, "at least one pulse required");
	assert!(n > 1, "at least one dimensions required");

	// Covers vectorization by up to 4.
	let mut iy = vec![0; n as usize + 3];
	exp_rotation(x, n, 1, b, k, spread);

	let yy = op_pvq_search_c(x, iy.as_mut_ptr(), k, n, arch);

	encode_pulses(iy.as_mut_ptr(), n, k, enc);

	if resynth != 0 {
		normalize_residual(&iy, x, n as usize, yy, gain);
		exp_rotation(x, n, -1, b, k, spread);
	}

	extract_collapse_mask(&iy, n as usize, b as usize)
}

#[no_mangle]
/**
Algebraic pulse decoder.

* `x` : Decoded normalised spectrum (returned)
* `n` : Number of samples to encode
* `k` : Number of pulses to use
* `dec` : Entropy decoder

Returns a mask indicating which blocks in the band received pulses.
*/
pub unsafe extern "C" fn alg_unquant(
	x: *mut celt_norm,
	n: c_int,
	k: c_int,
	spread: c_int,
	b: c_int,
	dec: &mut ec_ctx,
	gain: v16,
) -> c_uint {
	assert!(k > 0, "at least one pulse required");
	assert!(n > 1, "at least two dimensions required");

	let mut iy = vec![0; n as usize];
	let ryy = decode_pulses(iy.as_mut_ptr(), n, k, dec);
	normalize_residual(&iy, x, n as usize, ryy, gain);
	exp_rotation(x, n, -1, b, k, spread);
	extract_collapse_mask(&iy, n as usize, b as usize)
}

#[no_mangle]
pub unsafe extern "C" fn renormalise_vector(x: *mut celt_norm, n: c_int, gain: v16, _arch: c_int) {
	let n = n as usize;
	let x = slice::from_raw_parts_mut(x, n);
	let e = EPSILON + inner_prod_rs(x, x);
	let t = vshr32!(e, 2 * (k - 7));
	let g = mul16_16_p15!(rsqrt_norm!(t), gain);
	for xptr in x.iter_mut() {
		*xptr = extract16!(pshr32!(mult16_16!(g, *xptr), k + 1));
	}
}

#[no_mangle]
pub unsafe extern "C" fn stereo_itheta(
	x: *const celt_norm,
	y: *const celt_norm,
	stereo: c_int,
	n: c_int,
	_arch: c_int,
) -> c_int {
	let n = n as usize;
	let x = slice::from_raw_parts(x, n);
	let y = slice::from_raw_parts(y, n);

	let mut emid = EPSILON;
	let mut eside = EPSILON;
	if stereo != 0 {
		for (&xe, &ye) in x.iter().zip(y.iter()) {
			let m = add16!(shr16!(xe, 1), shr16!(ye, 1));
			let s = sub16!(shr16!(xe, 1), shr16!(ye, 1));
			emid = mac16_16!(emid, m, m);
			eside = mac16_16!(eside, s, s);
		}
	} else {
		emid += inner_prod_rs(x, x);
		eside += inner_prod_rs(y, y);
	}
	let mid = sqrt!(emid);
	let side = sqrt!(eside);
	let itheta = (0.5 + 16384.0 * FRAC_2_PI * fast_atan2(side, mid)).floor();
	itheta as c_int
}
