// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

/*!
Higher-level utilities for Ogg streams and files

This module provides access to opus encapsulated
inside the Ogg container format ([RFC 7845](https://tools.ietf.org/html/rfc7845)).
*/

use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use ::OpusError;

/// The ident header, as of
/// [RFC 7845, section 5.1](https://tools.ietf.org/html/rfc7845#section-5.1)
#[derive(Debug)]
pub struct IdentHeader {
	pub version :u8,
	pub output_channels :u8,
	pub pre_skip :u16,
	pub input_sample_rate :u32,
	pub output_gain :i16,
	pub channel_mapping_family :u8,
	// TODO channel mapping table:
	// https://tools.ietf.org/html/rfc7845#section-5.1.1
}

pub fn read_ident_header(packet :&[u8]) -> Result<IdentHeader, OpusError> {
	let opus_magic = &[0x4f, 0x70, 0x75, 0x73, 0x48, 0x65, 0x61, 0x64];
	if ! packet.starts_with(opus_magic) {
		// TODO return an error
		panic!("ERROR ident packet has no opus magic (TODO manage this via Result)");
	}
	let mut rdr = Cursor::new(&packet[opus_magic.len() ..]);
	let opus_version = try!(rdr.read_u8());
	// The version is internally separated into two halves:
	// The "major" and the "minor" half. We have to be forwards compatible
	// compatible with any version where the major half is 0.
	if opus_version >= 16 {
		// TODO return an error
		panic!("ERROR unsupported opus version (TODO manage this via Result)");
	}
	let output_channels = try!(rdr.read_u8());
	let pre_skip = try!(rdr.read_u16::<LittleEndian>());
	let input_sample_rate = try!(rdr.read_u32::<LittleEndian>());
	let output_gain = try!(rdr.read_i16::<LittleEndian>());
	let channel_mapping_family = try!(rdr.read_u8());
	// TODO read channel mapping table
	return Ok(IdentHeader {
		version : opus_version,
		output_channels : output_channels,
		pre_skip : pre_skip,
		input_sample_rate : input_sample_rate,
		output_gain : output_gain,
		channel_mapping_family : channel_mapping_family,
	});
}

// TODO read comment header
