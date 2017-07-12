// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use std::os::raw::*;

// Replaces mathops.c and also provides ilog functions

// TODO: implement fixed point stuff of this file

/*
The ilog functions live in opus as EC_ILOG macro,
which dispatches depending on the platform to BSR / CLZ
instruction intrinsics, or to a fast C implementation,
in ecintrin.{h,c}.
We can use libstd for the CLZ, which dispatches to
intrinsics as well :)
*/

/// Unsigned ilog function (Section 1.1.10.)
pub fn ilog(u :u32) -> u32 {
	32 - u.leading_zeros()
}

#[test]
fn test_ilog() {
	// Test values from the opus spec
	assert_eq!(ilog(0), 0);
	assert_eq!(ilog(1), 1);
	assert_eq!(ilog(2), 2);
	assert_eq!(ilog(3), 2);
	assert_eq!(ilog(4), 3);
	assert_eq!(ilog(7), 3);
}

#[no_mangle]
/// Compute floor(sqrt(_val)) with exact arithmetic.
pub extern "C" fn isqrt32(mut val: u32) -> c_uint {
	let mut bshift :c_int = (ilog(val) as c_int - 1) >> 1;
	let mut b :c_uint = 1 << bshift;
	let mut g :c_uint = 0;
	// Uses the second method from
	// http://www.azillionmonkeys.com/qed/sqroot.html
	// The main idea is to search for the largest binary digit b such that
	// (g+b)*(g+b) <= _val, and add it to the solution g.*/
	loop {
		let t :u32 = (((g as u32) << 1) + b) << bshift;
		if t <= val {
			g += b;
			val -= t;
		}
		b >>= 1;
		bshift -= 1;
		if bshift < 0 {
			return g;
		}
	}
}

#[test]
fn test_isqrt32() {
	// Some random test values
	assert_eq!(isqrt32(2), 1);
	assert_eq!(isqrt32(4), 2);
	assert_eq!(isqrt32(9), 3);
	assert_eq!(isqrt32(12), 3);
	assert_eq!(isqrt32(14), 3);
	assert_eq!(isqrt32(42246), 205);
}
