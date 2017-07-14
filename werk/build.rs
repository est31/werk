// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

fn main() {
	// Not sure why, but this is needed so that libopus
	// can "see" our exported functions; you'd get a linker
	// error when removing this.
	println!("cargo:rustc-link-lib=static=opus");
}
