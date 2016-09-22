// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

extern crate werk;
extern crate ogg;

use std::env;
use std::fs::File;
use ogg::PacketReader;
use werk::inside_ogg::*;

fn main() {
	run_decode();
}

fn run_decode() {
	macro_rules! try {
		($expr:expr) => (match $expr {
			$crate::std::result::Result::Ok(val) => val,
			$crate::std::result::Result::Err(err) => {
				panic!("Error: {:?}", err)
			}
		})
	}

	let file_path = env::args().nth(1).expect("Please specify a file to open via arg.");
	println!("Opening file: {}", file_path);
	let mut frdr = try!(File::open(file_path));
	let mut rdr = PacketReader::new(&mut frdr);

	let pck_ident = try!(rdr.read_packet());
	let pck_comment = try!(rdr.read_packet());
	let ident_hdr = try!(read_ident_header(&pck_ident.data));
	let comment_hdr = try!(read_comment_header(&pck_comment.data));

	println!("{:?}\n{:?}", ident_hdr, comment_hdr);
}
