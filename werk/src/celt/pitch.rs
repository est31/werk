// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use super::arch::*;
use super::entcode::celt_udiv;
use super::lpc::{_celt_autocorr, _celt_lpc};
use std::os::raw::*;
use std::ptr;
use std::slice;

fn find_best_pitch(xcorr: &[v32], y: &[v16], len: usize) -> (usize, usize) {
	let mut best_num = (-1.0, -1.0);
	let mut best_den = (0.0, 0.0);
	let mut best_pitch = (0, 1);

	let mut syy = 1.0;

	#[allow(unused_variables)]
	let xshift = (); // TODO obtain the correct fixed point value
	#[allow(unused_variables)]
	let yshift = (); // TODO obtain the correct fixed point value

	for v in y.iter().take(len) {
		syy = add32!(syy, shr32!(mult16_16!(*v, *v), yshift));
	}

	for (i, &xcorr_i) in xcorr.iter().enumerate() {
		if xcorr_i > 0.0 {
			let xcorr16 = extract16!(vshr32!(xcorr_i, xshift));
			let num = mul16_16_q15!(xcorr16, xcorr16);
			if mul16_32_q15!(num, best_den.1) > mul16_32_q15!(best_num.1, syy) {
				if mul16_32_q15!(num, best_den.0) > mul16_32_q15!(best_num.0, syy) {
					best_num.1 = best_num.0;
					best_den.1 = best_den.0;
					best_pitch.1 = best_pitch.0;
					best_num.0 = num;
					best_den.0 = syy;
					best_pitch.0 = i;
				} else {
					best_num.1 = num;
					best_den.1 = syy;
					best_pitch.1 = i;
				}
			}
		}
		syy += shr32!(mult16_16!(y[i + len], y[i + len]), yshift)
			- shr32!(mult16_16!(y[i], y[i]), yshift);
		syy = max32!(1.0, syy);
	}
	best_pitch
}

fn celt_fir5(x: &mut [v16], num: &[v16]) {
	let num0 = num[0];
	let num1 = num[1];
	let num2 = num[2];
	let num3 = num[3];
	let num4 = num[4];
	let mut mem0 = 0.0;
	let mut mem1 = 0.0;
	let mut mem2 = 0.0;
	let mut mem3 = 0.0;
	let mut mem4 = 0.0;
	for i in 0..x.len() {
		let mut sum = shl32!(extend32!(x[i]), SIG_SHIFT);
		sum = mac16_16!(sum, num0, mem0);
		sum = mac16_16!(sum, num1, mem1);
		sum = mac16_16!(sum, num2, mem2);
		sum = mac16_16!(sum, num3, mem3);
		sum = mac16_16!(sum, num4, mem4);
		mem4 = mem3;
		mem3 = mem2;
		mem2 = mem1;
		mem1 = mem0;
		mem0 = x[i];
		x[i] = round16!(sum, SIG_SHIFT);
	}
}

#[no_mangle]
pub extern "C" fn pitch_downsample(
	x: *mut *mut celt_sig,
	x_lp: *mut v16,
	len: c_int,
	c: c_int,
	arch: c_int,
) {
	let len = len as usize;
	let len2 = len >> 1;
	let mut x_lp = unsafe { slice::from_raw_parts_mut(x_lp, len2) };
	let x_0 = unsafe { slice::from_raw_parts(*x, len) };
	let x_1 = unsafe { slice::from_raw_parts(*(x.offset(1)), len) };
	x_lp[0] = shr32!(half32!(half32!(x_0[1]) + x_0[0]), shift);
	for i in 1..len2 {
		x_lp[i] = shr32!(
			half32!(half32!(x_0[2 * i - 1] + x_0[2 * i + 1]) + x_0[2 * i]),
			shift
		);
	}
	if c == 2 {
		x_lp[0] += shr32!(half32!(half32!(x_1[1]) + x_1[0]), shift);
		for i in 1..len2 {
			x_lp[i] += shr32!(
				half32!(half32!(x_1[2 * i - 1] + x_1[2 * i + 1]) + x_1[2 * i]),
				shift
			);
		}
	}
	let mut ac = [0.0; 5];
	_celt_autocorr(
		x_lp.as_ptr(),
		(&mut ac).as_mut_ptr(),
		ptr::null(),
		0,
		4,
		len as c_int >> 1,
		arch,
	);

	// Noise floor -40 dB
	ac[0] *= 1.0001;
	// Lag windowing
	for i in 1..5 {
		// ac[i] *= exp(-.5*(2*M_PI*.002*i)*(2*M_PI*.002*i));
		ac[i] -= ac[i] * (0.008 * i as f32) * (0.008 * i as f32);
	}

	let mut lpc = [0.0; 4];
	_celt_lpc((&mut lpc).as_mut_ptr(), (&mut ac).as_mut_ptr(), 4);
	let mut tmp = Q15ONE;
	for i in 0..4 {
		tmp *= 0.9;
		lpc[i] *= tmp;
	}
	let mut lpc2 = [0.0; 5];
	let c1 = qconst16!(0.8, 15);
	// Add a zero
	lpc2[0] = lpc[0] + qconst16!(0.8, SIG_SHIFT);
	lpc2[1] = lpc[1] + mul16_16_q15!(c1, lpc[0]);
	lpc2[2] = lpc[2] + mul16_16_q15!(c1, lpc[1]);
	lpc2[3] = lpc[2] + mul16_16_q15!(c1, lpc[2]);
	lpc2[4] = mul16_16_q15!(c1, lpc[3]);

	celt_fir5(&mut x_lp, &lpc2);
}

#[no_mangle]
pub extern "C" fn pitch_search(
	x_lp: *const v16,
	y: *const v16,
	len: c_int,
	max_pitch: c_int,
	pitch: &mut c_int,
	arch: c_int,
) {
	assert!(len > 0);
	assert!(max_pitch > 0);

	let lag = len + max_pitch;

	let len2 = len as usize >> 1;
	let len4 = len as usize >> 2;
	let lag2 = lag as usize >> 1;
	let lag4 = lag as usize >> 2;

	let max_pitch = max_pitch as usize;

	let x_lp = unsafe { slice::from_raw_parts(x_lp, len2) };
	let y = unsafe { slice::from_raw_parts(y, lag2) };

	let mut x_lp4 = Vec::with_capacity(len4);
	let mut y_lp4 = Vec::with_capacity(lag4);
	let mut xcorr = vec![0.0; max_pitch >> 1];

	// Downsample by 2 again
	for j in 0..len4 {
		x_lp4.push(x_lp[2 * j]);
	}
	for j in 0..lag4 {
		y_lp4.push(y[2 * j]);
	}

	// Coarse search with 4x decimation

	celt_pitch_xcorr_c(
		x_lp4.as_ptr(),
		y_lp4.as_ptr(),
		xcorr.as_mut_ptr(),
		len >> 2,
		max_pitch as c_int >> 2,
		arch,
	);

	let best_pitch = find_best_pitch(&xcorr[0..max_pitch >> 2], &y_lp4, len4);

	// Finer search with 2x decimation

	for i in 0..max_pitch >> 1 {
		if (i as isize - 2 * best_pitch.0 as isize).abs() > 2
			&& (i as isize - 2 * best_pitch.1 as isize).abs() > 2
		{
			continue;
		}
		let sum = inner_prod_rs(x_lp, &y[i..(i + len2)]);
		xcorr[i] = max32!(-1.0, sum);
	}

	let best_pitch = find_best_pitch(&xcorr, &y, len2);

	// Refine by pseudo-interpolation
	let offset = if best_pitch.0 > 0 && best_pitch.0 < (max_pitch >> 1) - 1 {
		let a = xcorr[best_pitch.0 - 1];
		let b = xcorr[best_pitch.0];
		let c = xcorr[best_pitch.0 + 1];

		if (c - a) > mul16_32_q15!(qconst16!(0.7, 15), b - a) {
			1
		} else if (a - c) > mul16_32_q15!(qconst16!(0.7, 15), b - c) {
			-1
		} else {
			0
		}
	} else {
		0
	};
	*pitch = 2 * best_pitch.0 as c_int - offset;
}

fn compute_pitch_gain(xy: v32, xx: v32, yy: v32) -> v16 {
	xy / (1.0 + xx * yy).sqrt()
}

#[no_mangle]
pub extern "C" fn remove_doubling(
	x: *const v16,
	max_period: c_int,
	min_period: c_int,
	n: c_int,
	t0_: &mut c_int,
	prev_period: c_int,
	prev_gain: v16,
	_arch: c_int,
) -> v16 {
	static SECOND_CHECK: [u32; 16] = [0, 0, 3, 2, 3, 2, 5, 2, 3, 2, 3, 2, 5, 2, 3, 2];

	let min_period_0 = min_period;
	let max_period = (max_period / 2) as isize;
	let min_period = min_period / 2;
	*t0_ /= 2;
	let n = (n / 2) as isize;

	let xb = unsafe { slice::from_raw_parts(x, (n + max_period) as usize) };

	macro_rules! x {
		($i:expr) => {
			xb[($i as isize + max_period) as usize]
		};
	}
	macro_rules! xsl {
		() => {
			&xb[max_period..]
		};
		($a:expr) => {
			&xb[($a as isize + max_period) as usize..]
		};
		($a:expr, $b:expr) => {
			&xb[(($a as isize + max_period) as usize)
				..(($a as isize + max_period + $b as isize) as usize)]
		};
	}

	if *t0_ as isize >= max_period {
		*t0_ = (max_period - 1) as c_int;
	}

	let t0 = *t0_;
	let mut t = *t0_ as usize;

	let (xx, mut xy) = dual_inner_prod_rs(xsl![0, n], xsl![0, n], xsl![(-t0), n]);
	let mut yy_lookup = Vec::with_capacity(max_period as usize + 1);
	yy_lookup.push(0.0);
	let mut yy = xx;
	for i in 1..=max_period {
		yy += mult16_16!(x![-i], x![-i]) - mult16_16!(x![n - i], x![n - i]);
		yy_lookup.push(max32!(0.0, yy));
	}
	yy = yy_lookup[t0 as usize];
	let mut best_xy = xy;
	let mut best_yy = yy;
	let mut g = compute_pitch_gain(xy, xx, yy);
	let g0 = g;
	// Look for any pitch at T/k
	for k in 2u32..16 {
		let t1 = celt_udiv(2 * t0 as u32 + k, 2 * k) as i32;
		if t1 < min_period as i32 {
			break;
		}
		// Look for another strong correlation at t1b
		let t1b = if k == 2 {
			if t1 + t0 > max_period as i32 {
				t0
			} else {
				t0 + t1
			}
		} else {
			celt_udiv(2 * SECOND_CHECK[k as usize] * t0 as u32 + k, 2 * k) as i32
		};
		let (xyp, xy2) = dual_inner_prod_rs(xsl![0, n], xsl![-t1, n], xsl![-t1b, n]);
		xy = half32!(xyp + xy2);
		yy = half32!(yy_lookup[t1 as usize] + yy_lookup[t1b as usize]);
		let g1 = compute_pitch_gain(xy, xx, yy);
		let cont = if (t1 - prev_period).abs() <= 1 {
			prev_gain
		} else if (t1 - prev_period).abs() <= 2 && 5 * k * k < t0 as u32 {
			half16!(prev_gain)
		} else {
			0.0
		};
		let mut thresh = max16!(
			qconst16!(0.3, 15),
			mul16_16_q15!(qconst16!(0.7, 15), g0) - cont
		);
		// Bias against very high pitch (very short period) to avoid
		// false positives due to short-term correlation
		if t1 < 3 * min_period {
			thresh = max16!(
				qconst16!(0.4, 15),
				mul16_16_q15!(qconst16!(0.85, 15), g0) - cont
			);
		} else if t1 < 2 * min_period {
			thresh = max16!(
				qconst16!(0.5, 15),
				mul16_16_q15!(qconst16!(0.9, 15), g0) - cont
			);
		}
		if g1 > thresh {
			best_xy = xy;
			best_yy = yy;
			t = t1 as usize;
			g = g1;
		}
	}
	best_xy = max32!(0.0, best_xy);
	let mut pg = if best_yy <= best_xy {
		Q15ONE
	} else {
		shr32!(frac_div32!(best_xy, best_yy + 1.0), 16)
	};
	let mut xcorr = [0.0; 3];
	for k in 0..3 {
		xcorr[k] = inner_prod_rs(xsl![0, n], xsl![-((t + k) as isize - 1), n]);
	}
	let offset = if xcorr[2] - xcorr[0] > mul16_32_q15!(qconst16!(0.7, 15), xcorr[1] - xcorr[0]) {
		1
	} else if xcorr[0] - xcorr[2] > mul16_32_q15!(qconst16!(0.7, 15), xcorr[1] - xcorr[2]) {
		-1
	} else {
		0
	};
	if pg > g {
		pg = g;
	}
	*t0_ = 2 * t as c_int + offset;
	if *t0_ < min_period_0 {
		*t0_ = min_period_0;
	}
	pg
}

#[no_mangle]
pub extern "C" fn celt_pitch_xcorr_c(
	x: *const v16,
	y: *const v16,
	xcorr: *mut v32,
	len: c_int,
	max_pitch: c_int,
	arch: c_int,
) {
	// Unrolled version of the pitch correlation -- runs faster on x86 and ARM
	assert!(max_pitch > 0);
	assert_eq!(x as usize & 3, 0);

	let len = len as usize;
	let max_pitch = max_pitch as usize;

	let xcorr = unsafe { slice::from_raw_parts_mut(xcorr, max_pitch) };
	let x = unsafe { slice::from_raw_parts(x, len) };

	let mut i = 0;
	loop {
		if i + 3 >= max_pitch {
			break;
		}
		let mut sum = [0.0; 4];
		let yoff = unsafe { y.add(i) };
		xcorr_kernel_rs(x, yoff, &mut sum, len, arch);
		xcorr[i] = sum[0];
		xcorr[i + 1] = sum[1];
		xcorr[i + 2] = sum[2];
		xcorr[i + 3] = sum[3];
		i += 4;
	}
	// In case max_pitch isn't a multiple of 4, do unrolled version
	while i < max_pitch {
		let yoff = unsafe { slice::from_raw_parts(y.add(i), len) };
		let sum = inner_prod_rs(x, yoff);
		xcorr[i] = sum;
		i += 1;
	}
}

// Some functions from pitch.h

#[inline]
pub fn xcorr_kernel_rs(x: &[v16], y: *const v16, sum: &mut [v32; 4], len: usize, _arch: c_int) {
	let y = unsafe { slice::from_raw_parts(y, len + 3) };
	assert!(len >= 3);
	let mut y0 = y[0];
	let mut y1 = y[1];
	let mut y2 = y[2];
	let mut y3;
	macro_rules! declare_sum {
		($name:ident, $i:expr, $yj:expr; $va:expr, $vb:expr, $vc:expr, $vd:expr) => {
			macro_rules! $name {
				($xe:expr, $ye:expr) => {
					let tmp = $xe[$i];
					$yj = $ye[$i];
					sum[0] = mac16_16!(sum[0], tmp, $va);
					sum[1] = mac16_16!(sum[1], tmp, $vb);
					sum[2] = mac16_16!(sum[2], tmp, $vc);
					sum[3] = mac16_16!(sum[3], tmp, $vd);
				};
			}
		};
	}
	declare_sum!(sum_y0, 0, y3; y0, y1, y2, y3);
	declare_sum!(sum_y1, 1, y0; y1, y2, y3, y0);
	declare_sum!(sum_y2, 2, y1; y2, y3, y0, y1);
	declare_sum!(sum_y3, 3, y2; y3, y0, y1, y2);

	let mut it = x.chunks(4).zip(y[3..].chunks(4));
	let (last_x, last_y) = it.next_back().unwrap();

	for (xe, ye) in it {
		sum_y0!(xe, ye);
		sum_y1!(xe, ye);
		sum_y2!(xe, ye);
		sum_y3!(xe, ye);
	}
	// Handle the last item
	match last_x.len() {
		1 => {
			sum_y0!(last_x, last_y);
		}
		2 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
		}
		3 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
			sum_y2!(last_x, last_y);
		}
		4 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
			sum_y2!(last_x, last_y);
			sum_y3!(last_x, last_y);
		}
		_ => unreachable!(),
	}
}

// dual_inner_prod_c
#[inline]
pub fn dual_inner_prod_rs(x: &[v16], y01: &[v16], y02: &[v16]) -> (v32, v32) {
	let mut xy01 = 0.0;
	let mut xy02 = 0.0;
	for (xe, (y01e, y02e)) in x.iter().zip(y01.iter().zip(y02.iter())) {
		xy01 = mac16_16!(xy01, *xe, *y01e);
		xy02 = mac16_16!(xy01, *xe, *y02e);
	}
	(xy01, xy02)
}

// celt_inner_prod_c
#[inline]
pub fn inner_prod_rs(x: &[v16], y: &[v16]) -> v16 {
	let mut xy = 0.0;
	for (xe, ye) in x.iter().zip(y.iter()) {
		xy = mac16_16!(xy, *xe, *ye);
	}
	xy
}
