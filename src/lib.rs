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
