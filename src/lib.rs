// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

#![deny(unsafe_code)]

/*!
An `opus` decoder, written in Rust.

This crate implements an `opus` decoder. opus is specified in
[RFC 6716](https://tools.ietf.org/html/rfc6716).

Currently this is in the early stages of development,
nothing actually works yet.
*/

extern crate byteorder;
#[cfg(feature = "ogg")]
extern crate ogg;

pub mod framing;

#[cfg(feature = "ogg")]
pub mod inside_ogg;

use std::io;
use ogg::OggReadError;

#[derive(Debug)]
pub enum OpusError {
	IoError(io::Error),
	#[cfg(feature = "ogg")]
	OggError(OggReadError),
}


impl std::error::Error for OpusError {
	fn description(&self) -> &str {
		match self {
			&OpusError::IoError(_) => "IO error",
			#[cfg(feature = "ogg")]
			&OpusError::OggError(ref e) => e.description(),
		}
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match self {
			&OpusError::IoError(ref err) => Some(err as &std::error::Error),
			_ => None
		}
	}
}

impl std::fmt::Display for OpusError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(fmt, "{}", std::error::Error::description(self))
	}
}

impl From<io::Error> for OpusError {
	fn from(err :io::Error) -> OpusError {
		OpusError::IoError(err)
	}
}

#[cfg(feature = "ogg")]
impl From<OggReadError> for OpusError {
	fn from(err :OggReadError) -> OpusError {
		OpusError::OggError(err)
	}
}

/// Signed ilog function (Spec section 1.1.10.)
fn iilog(val :i64) -> u8 {
	let mut ret :u8 = 0;
	let mut v = val;
	while v > 0 {
		ret += 1;
		v = v >> 1;
	}
	return ret;
}

#[test]
fn test_iilog() {
	// Test values from the opus spec
	assert_eq!(iilog(-1), 0);
	assert_eq!(iilog(0), 0);
	assert_eq!(iilog(1), 1);
	assert_eq!(iilog(2), 2);
	assert_eq!(iilog(3), 2);
	assert_eq!(iilog(4), 3);
	assert_eq!(iilog(7), 3);
}

/// Unsigned ilog function (Spec section 1.1.10.)
fn uilog(val :u64) -> u8 {
	let mut ret :u8 = 0;
	let mut v = val;
	while v > 0 {
		ret += 1;
		v = v >> 1;
	}
	return ret;
}

#[test]
fn test_uilog() {
	// Test values from the opus spec
	assert_eq!(uilog(0), 0);
	assert_eq!(uilog(1), 1);
	assert_eq!(uilog(2), 2);
	assert_eq!(uilog(3), 2);
	assert_eq!(uilog(4), 3);
	assert_eq!(uilog(7), 3);
}
