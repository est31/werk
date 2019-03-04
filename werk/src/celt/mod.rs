// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

// For the time being, until we are completely Rust,
// allow bad style
#![allow(non_camel_case_types)]

// OPUS_CLEAR macro:
macro_rules! slice_clear {
	($slice:expr) => {{
		for e in $slice.iter_mut() {
			*e = 0.0;
			}
		}};
}

#[macro_use]
pub mod arch;
pub mod bands;
pub mod cwrs;
pub mod entcode;
pub mod entdec;
pub mod laplace;
pub mod lpc;
pub mod mathops;
pub mod pitch;
pub mod vq;
