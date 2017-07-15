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

// NOTE: we'll convert the macros to proper functions
// once #[repr(transparent)] is available:
// https://github.com/rust-lang/rust/issues/43036
// Until then we either have the choice of using stupid
// traits, or risking stuff.
// We rather have macros for now like C does :)

pub type v16 = f32;
pub type v32 = f32;
pub type v64 = f32;

pub const SIG_SHIFT :usize = 12;

// TODO: fill in here

macro_rules! extend32 {
	($a:expr) => {
		$a
	}
}

macro_rules! shr16 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

macro_rules! shl16 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

macro_rules! shr32 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

macro_rules! shl32 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

// TODO: fill in here

macro_rules! round16 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

macro_rules! sround16 {
	($a:expr, $_sh:expr) => {
		$a
	}
}

// TODO: fill in here

macro_rules! mult16_16 {
	($a:expr, $b:expr) => {
		($a as v32) * ($b as v32)
	}
}

macro_rules! mac16_16 {
	($c:expr, $a:expr, $b:expr) => {
		$c + ($a as v32) * ($b as v32)
	}
}

// TODO: fill in here

macro_rules! mul32_32_q31 {
	($a:expr, $b:expr) => {
		$a * $b
	}
}

// TODO: fill in here

macro_rules! mul16_16_q15 {
	($a:expr, $b:expr) => {
		$a * $b
	}
}

// TODO: fill in here

macro_rules! frac_div32 {
	($a:expr, $b: expr) => {
		$a / $b
	}
}
