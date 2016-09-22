// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

extern crate werk;

use std::env;

fn main() {
	run_decode();
}

fn run_decode() {
	let file_path = env::args().nth(1).expect("Please specify a file to open via arg.");
	println!("Opening file: {}", file_path);
	// TODO
}
