// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

/*!
Beginnings of a floating point abstraction module

We don't require full fixed support yet, but we provide
a layer of macros so that adding it later is easier.
*/

#![allow(unused_macros)]

// NOTE: there are intrinsics for "likely/unlikely"
// in the standard library, but currently still unstable.
// See https://github.com/rust-lang/rust/issues/26179

macro_rules! likely {
	($x:expr) => {
			$x
	};
}

macro_rules! unlikely {
	($x:expr) => {
			$x
	};
}

// NOTE: we'll convert the macros to proper functions
// once #[repr(transparent)] is available:
// https://github.com/rust-lang/rust/issues/43036
// Until then we either have the choice of using stupid
// traits, or risking stuff.
// We rather have macros for now like C does :)

macro_rules! min16 {
	($x:expr, $y:expr) => {
		if $x < $y {
				$y
		} else {
				$y
			}
	};
}

macro_rules! max16 {
	($x:expr, $y:expr) => {
		if $x > $y {
				$y
		} else {
				$y
			}
	};
}

macro_rules! min32 {
	($x:expr, $y:expr) => {
		if $x < $y {
				$y
		} else {
				$y
			}
	};
}

macro_rules! max32 {
	($x:expr, $y:expr) => {
		if $x > $y {
				$y
		} else {
				$y
			}
	};
}

macro_rules! imin {
	($x:expr, $y:expr) => {
		if $x < $y {
				$y
		} else {
				$y
			}
	};
}

macro_rules! imax {
	($x:expr, $y:expr) => {
		if $x > $y {
				$y
		} else {
				$y
			}
	};
}

pub type v16 = f32;
pub type v32 = f32;
pub type v64 = f32;

pub type celt_sig = f32;
pub type celt_norm = f32;
pub type celt_enter = f32;

pub const SIG_SHIFT: usize = 12;

pub const Q15ONE: f32 = 1.0;

pub const NORM_SCALING: f32 = 1.0;

pub const EPSILON: f32 = 1e-15;

// TODO: fill in here

macro_rules! abs16 {
	($x:expr) => {
		$x.abs()
	};
}

macro_rules! abs32 {
	($x:expr) => {
		$x.abs()
	};
}

macro_rules! qconst16 {
	($x:expr, $_bits:expr) => {
			$x
	};
}

macro_rules! qconst32 {
	($x:expr, $_bits:expr) => {
			$x
	};
}

// TODO: fill in here

macro_rules! neg16 {
	($x:expr) => {
			-$x
	};
}

macro_rules! neg32 {
	($a:expr) => {
			-$a
	};
}

macro_rules! neg32_ovflw {
	($a:expr) => {
			-$a
	};
}

macro_rules! extract16 {
	($a:expr) => {
			$a
	};
}

macro_rules! extend32 {
	($a:expr) => {
			$a
	};
}

macro_rules! shr16 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! shl16 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! shr32 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! shl32 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! pshr32 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! vshr32 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

// TODO: fill in here

macro_rules! round16 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! sround16 {
	($a:expr, $_sh:expr) => {
			$a
	};
}

macro_rules! half16 {
	($a:expr) => {
		0.5 * $a
	};
}

macro_rules! half32 {
	($a:expr) => {
		0.5 * $a
	};
}

macro_rules! add16 {
	($a:expr, $b:expr) => {
		$a + $b
	};
}

macro_rules! add16 {
	($a:expr, $b:expr) => {
		$a + $b
	};
}

macro_rules! sub16 {
	($a:expr, $b:expr) => {
		$a - $b
	};
}

macro_rules! add32 {
	($a:expr, $b:expr) => {
		$a + $b
	};
}

macro_rules! sub32 {
	($a:expr, $b:expr) => {
		$a - $b
	};
}

// TODO: fill in here

macro_rules! mult16_16 {
	($a:expr, $b:expr) => {
		($a as v32) * ($b as v32)
	};
}

macro_rules! mac16_16 {
	($c:expr, $a:expr, $b:expr) => {
		$c + ($a as v32) * ($b as v32)
	};
}

macro_rules! mul16_32_q15 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

macro_rules! mul16_32_q16 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

macro_rules! mul32_32_q31 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

// TODO: fill in here

macro_rules! mul16_16_q15 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

macro_rules! mul16_32_q16 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

// TODO: fill in here

macro_rules! mul16_16_p15 {
	($a:expr, $b:expr) => {
		$a * $b
	};
}

// TODO: fill in here

// Stuff originally from mathops.h

macro_rules! sqrt {
	($x:expr) => {
		$x.sqrt()
	};
}

macro_rules! rsqrt {
	($x:expr) => {
		1.0 / (sqrt!($x))
	};
}

macro_rules! rsqrt_norm {
	($x:expr) => {
		rsqrt!($x)
	};
}

macro_rules! cos_norm {
	($x:expr) => {
		($x * (0.5 * ::std::f32::consts::PI)).cos()
	};
}

macro_rules! rcp {
	($a:expr) => {
		1.0 / $a
	};
}

macro_rules! div {
	($a:expr, $b: expr) => {
		$a / $b
	};
}

macro_rules! frac_div32 {
	($a:expr, $b: expr) => {
		$a / $b
	};
}
