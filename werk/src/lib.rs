// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

#[allow(unused_extern_crates)]
extern crate werk_sys;

pub mod celt;
pub mod repacketizer;

/// Workaround for https://github.com/rust-lang/rust/issues/18807
pub extern "C" fn so_dead() {
	println!();
}
