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

#[derive(Debug)]
pub struct ChannelMappingTable {
	pub stream_count :u8,
	pub coupled_stream_count :u8,
	pub channel_mapping :Vec<u8>,
}

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
	pub channel_mapping_table :Option<ChannelMappingTable>,
}

pub fn read_ident_header(packet :&[u8]) -> Result<IdentHeader, OpusError> {
	// The magic is "OpusHead".
	let opus_magic = &[0x4f, 0x70, 0x75, 0x73, 0x48, 0x65, 0x61, 0x64];
	if !packet.starts_with(opus_magic) {
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
	let channel_mapping_table =	if channel_mapping_family == 0 {
		None
	} else {
		let stream_count = try!(rdr.read_u8());
		let coupled_stream_count = try!(rdr.read_u8());
		let mut channel_mapping = Vec::with_capacity(output_channels as usize);
		for _ in 0 .. output_channels {
			channel_mapping.push(try!(rdr.read_u8()));
		}
		Some(ChannelMappingTable {
			stream_count : stream_count,
			coupled_stream_count : coupled_stream_count,
			channel_mapping : channel_mapping,
		})
	};
	return Ok(IdentHeader {
		version : opus_version,
		output_channels : output_channels,
		pre_skip : pre_skip,
		input_sample_rate : input_sample_rate,
		output_gain : output_gain,
		channel_mapping_family : channel_mapping_family,
		channel_mapping_table : channel_mapping_table,
	});
}

#[derive(Debug)]
pub struct CommentHeader {
	vendor :String,
	comment_list :Vec<(String, String)>,
}

pub fn read_comment_header(packet :&[u8]) -> Result<CommentHeader, OpusError> {
	// The magic is "OpusTags".
	let comment_magic = &[0x4f, 0x70, 0x75, 0x73, 0x54, 0x61, 0x67, 0x73];
	if !packet.starts_with(comment_magic) {
		// TODO return an error
		panic!("ERROR comment packet has no opus magic (TODO manage this via Result)");
	}
	let mut rdr = Cursor::new(&packet[comment_magic.len() ..]);
	use std::io::Read;

	// First read the vendor string
	let vendor_length = try!(rdr.read_u32::<LittleEndian>()) as usize;
	// TODO fix this, we initialize memory for NOTHING!!! Out of some reason, this is seen as "unsafe" by rustc.
	let mut vendor_buf = vec![0; vendor_length];
	try!(rdr.read_exact(&mut vendor_buf));
	// TODO use try macro instead
	let vendor = String::from_utf8(vendor_buf).expect("UTF-8 decode error");

	// Now read the comments
	let comment_count = try!(rdr.read_u32::<LittleEndian>()) as usize;
	let mut comment_list = Vec::with_capacity(comment_count);
	for _ in 0 .. comment_count {
		let comment_length = try!(rdr.read_u32::<LittleEndian>()) as usize;
		// TODO fix this, we initialize memory for NOTHING!!! Out of some reason, this is seen as "unsafe" by rustc.
		let mut comment_buf = vec![0; comment_length];
		try!(rdr.read_exact(&mut comment_buf));
		// TODO use try macro instead
		let comment = String::from_utf8(comment_buf).expect("UTF-8 decode error");
		let eq_idx = match comment.find("=") {
			Some(k) => k,
			// Return an error here for closer compliance with the spec.
			// It appears that some files have fields without a = sign in the comments.
			// Well there is not much we can do but gracefully ignore their stuff.
			None => continue
		};
		let (key_eq, val) = comment.split_at(eq_idx + 1);
		let (key, _) = key_eq.split_at(eq_idx);
		comment_list.push((String::from(key), String::from(val)));
	}
	let hdr :CommentHeader = CommentHeader {
		vendor : vendor,
		comment_list : comment_list,
	};
	return Ok(hdr);
}
