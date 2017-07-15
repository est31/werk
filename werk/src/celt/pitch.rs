// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use std::os::raw::*;
use std::slice;
use super::arch::*;

extern "C" {
	pub fn celt_pitch_xcorr_c(_x :* const v16, _y :* const v16,
		xcorr :* mut v32,
		len :c_int,
		max_pitch :c_int,
		arch :c_int);
}

// TODO implement content of pich.c
// Here we only implement a function from pitch.h

#[inline]
pub fn xcorr_kernel_rs(x :&[v16], y :* const v16, sum :&mut [v32; 4],
		len :usize, _arch :c_int) {
	let y = unsafe {
		slice::from_raw_parts(y, len + 3)
	};
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
				}
			}
		}
	}
	declare_sum!(sum_y0, 0, y3; y0, y1, y2, y3);
	declare_sum!(sum_y1, 1, y0; y1, y2, y3, y0);
	declare_sum!(sum_y2, 2, y1; y2, y3, y0, y1);
	declare_sum!(sum_y3, 3, y2; y3, y0, y1, y2);

	let mut it = x.chunks(4).zip(y[3 ..].chunks(4));
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
		},
		2 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
		},
		3 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
			sum_y2!(last_x, last_y);
		},
		4 => {
			sum_y0!(last_x, last_y);
			sum_y1!(last_x, last_y);
			sum_y2!(last_x, last_y);
			sum_y3!(last_x, last_y);
		},
		_ => unreachable!(),
	}
}
