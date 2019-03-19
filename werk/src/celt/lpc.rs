// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

#[allow(unused_imports)]
use super::arch::SIG_SHIFT;
use super::arch::{v16, v32};
use super::pitch::{celt_pitch_xcorr_c, xcorr_kernel_rs};
use std::os::raw::*;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn _celt_lpc(lpc: *mut v16, ac: *const v32, p: c_int) {
	let p = p as usize;
	let lpc = slice::from_raw_parts_mut(lpc, p);
	let ac = slice::from_raw_parts(ac, p + 1);
	let mut error = ac[0];
	slice_clear!(lpc);
	if ac[0] != 0.0 {
		for i in 0..p {
			// Sum up this iteration's reflection coefficient
			let mut rr = 0.0;
			for j in 0..i {
				rr += mul32_32_q31!(lpc[j], ac[i - j]);
			}
			rr += shr32!(ac[i + 1], 3);
			let r = -frac_div32!(shl32!(rr, 3), error);
			// Update LPC coefficients and total error
			lpc[i] = shr32!(r, 3);
			for j in 0..(i + 1) >> 1 {
				let tmp1 = lpc[j];
				let tmp2 = lpc[i - 1 - j];
				lpc[j] = tmp1 + mul32_32_q31!(r, tmp2);
				lpc[i - 1 - j] = tmp2 + mul32_32_q31!(r, tmp1);
			}
			error -= mul32_32_q31!(mul32_32_q31!(r, r), error);
			// Bail out once we get 30 dB gain
			if error < 0.001 * ac[0] {
				break;
			}
		}
	}
	// No-op except for fixed point
	for e in lpc.iter_mut() {
		*e = round16!(*e, 16);
	}
}

#[no_mangle]
pub unsafe extern "C" fn celt_fir_c(
	x: *const v16,
	num: *const v16,
	y: *mut v16,
	n: c_int,
	ord: c_int,
	arch: c_int,
) {
	let ord = ord as usize;
	let n = n as usize;

	assert_ne!(x, y);

	let num = slice::from_raw_parts(num, ord);
	let y = slice::from_raw_parts_mut(y, n);

	let mut rnum = Vec::with_capacity(ord);
	for i in 0..ord {
		rnum.push(num[ord - i - 1]);
	}
	macro_rules! sle {
		($e:expr) => {
			shl32!(extend32!($e), SIG_SHIFT)
		};
	}
	for i4 in 0..(n - 3) / 4 {
		let i = i4 << 2;
		let mut sum = {
			let x = slice::from_raw_parts(x, n);
			[sle!(x[i]), sle!(x[i + 1]), sle!(x[i + 2]), sle!(x[i + 3])]
		};
		// The x.offset below can be negative, therefore overflow.
		// Due to this, we can't convert x to a slice, not without
		// passing x - ord to this fn instead of x.
		xcorr_kernel_rs(&rnum, x.add(i - ord), &mut sum, ord, arch);

		y[i] = round16!(sum[0], SIG_SHIFT);
		y[i + 1] = round16!(sum[1], SIG_SHIFT);
		y[i + 2] = round16!(sum[2], SIG_SHIFT);
		y[i + 3] = round16!(sum[3], SIG_SHIFT);
	}
	for i in 4 * ((n - 3) / 4)..n {
		let mut sum = sle!(*x.add(i));
		for j in 0..ord {
			sum = mac16_16!(sum, rnum[j], *x.add(i + j - ord));
		}
		y[i] = round16!(sum, SIG_SHIFT);
	}
}

#[no_mangle]
pub unsafe extern "C" fn celt_iir(
	x: *const v32,
	den: *const v16,
	yp: *mut v16,
	n: c_int,
	ord: c_int,
	mem: *mut v16,
	arch: c_int,
) {
	let ord = ord as usize;
	let n = n as usize;

	// TODO: find out whether ord >= 3 because we require this here,
	// as we access den with indices of up to 2.
	// If its not the case, we'll have to give the den slice a size of
	// max(ord, 3) instead.
	// This assert makes it hopefully possible to optimize out the
	// bounds checks in the loop below.
	assert!(ord >= 3);

	let x = slice::from_raw_parts(x, n);
	let den = slice::from_raw_parts(den, ord);
	let yp = slice::from_raw_parts_mut(yp, n);
	let mem = slice::from_raw_parts_mut(mem, ord);
	let mut rden = Vec::with_capacity(ord);
	let mut y = Vec::with_capacity(ord + n);
	for i in 0..ord {
		rden.push(den[ord - i - 1]);
	}
	for i in 0..ord {
		y.push(-mem[ord - i - 1]);
	}
	for _ in 0..n {
		y.push(0.0);
	}
	for i4 in 0..(n - 3) / 4 {
		let i = i4 << 2;
		// Unroll by 4 as if it were an FIR filter
		let mut sum = [x[i], x[i + 1], x[i + 2], x[i + 3]];
		xcorr_kernel_rs(&rden, y.as_ptr().add(i), &mut sum, ord, arch);

		// Patch up the result to compensate for the fact that this is an IIR
		y[i + ord] = -sround16!(sum[0], SIG_SHIFT);
		yp[i] = sum[0];

		sum[1] = mac16_16!(sum[1], y[i + ord], den[0]);
		y[i + ord + 1] = -sround16!(sum[1], SIG_SHIFT);
		yp[i + 1] = sum[1];

		sum[2] = mac16_16!(sum[2], y[i + ord + 1], den[0]);
		sum[2] = mac16_16!(sum[2], y[i + ord], den[1]);
		y[i + ord + 2] = -sround16!(sum[2], SIG_SHIFT);
		yp[i + 1] = sum[2];

		sum[3] = mac16_16!(sum[3], y[i + ord + 2], den[0]);
		sum[3] = mac16_16!(sum[3], y[i + ord + 1], den[1]);
		sum[3] = mac16_16!(sum[3], y[i + ord], den[2]);
		y[i + ord + 3] = -sround16!(sum[3], SIG_SHIFT);
		yp[i + 3] = sum[3];
	}
	for i in 4 * ((n - 3) / 4)..n {
		let mut sum = x[i];
		for j in 0..ord {
			sum -= mult16_16!(rden[j], y[i + j]);
		}
		y[i + ord] = sround16!(sum, SIG_SHIFT);
		yp[i] = sum;
	}
	for i in 0..ord {
		mem[i] = yp[n - i - 1];
	}
}

#[no_mangle]
pub unsafe extern "C" fn _celt_autocorr(
	x: *const v16,
	ac: *mut v16,
	window: *const v16,
	overlap: c_int,
	lag: c_int,
	n: c_int,
	arch: c_int,
) -> c_int {
	let fast_n = n - lag;

	let overlap = overlap as usize;
	let n = n as usize;

	let x = slice::from_raw_parts(x, n);
	let window = slice::from_raw_parts(window, overlap);

	let mut xx = Vec::with_capacity(n);
	let xptr = if overlap == 0 {
		x
	} else {
		xx.extend_from_slice(x);
		for i in 0..overlap {
			xx[i] = mul16_16_q15!(x[i], window[i]);
			xx[n - i - 1] = mul16_16_q15!(x[n - i - 1], window[i]);
		}
		&xx
	};
	celt_pitch_xcorr_c(xptr.as_ptr(), xptr.as_ptr(), ac, fast_n, lag + 1, arch);
	let lag = lag as usize;
	let ac = slice::from_raw_parts_mut(ac, lag);
	for k in 0..lag {
		let mut d = 0.0;
		for i in k + (fast_n as usize)..n {
			d = mac16_16!(d, xptr[i], xptr[i - k]);
		}
		ac[k] += d;
	}
	0
}
