// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

use super::arch::v32;
use super::entcode::ec_ctx;
use std::os::raw::*;

extern "C" {
	pub fn encode_pulses(y: *const c_int, n: c_int, k: c_int, enc: *mut ec_ctx);
}
extern "C" {
	pub fn decode_pulses(y: *mut c_int, n: c_int, k: c_int, dec: *mut ec_ctx) -> v32;
}
