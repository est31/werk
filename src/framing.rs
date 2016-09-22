// Opus decoder written in Rust
//
// Copyright (c) 2016 the contributors.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

/*!
Framing layer of opus

See [section 3 of RFC 6716](https://tools.ietf.org/html/rfc6716#section-3).

Self delimiting framing (as of appendix B) is not supported (yet).
*/

/// The TOC byte that every packet includes
pub struct TocByte(u8);

pub enum PacketMode {
	SilkOnly,
	Hybrid,
	CeltOnly,
}

/// The bandwidth configurations opus supports
///
/// See table 1 of RFC 6716
pub enum Bandwidth {
	NarrowBand,
	MediumBand,
	Wideband,
	SuperWideband,
	Fullband,
}

pub enum FrameSize {
	Ms2p5,
	Ms5,
	Ms10,
	Ms20,
	Ms40,
	Ms60,
}

impl TocByte {
	pub fn get_mode(&self) -> PacketMode {
		// See "Table 2: TOC Byte Configuration Parameters"
		let config = self.0 & 63;
		if config <= 11 {
			return PacketMode::SilkOnly;
		}
		if config <= 15 {
			return PacketMode::Hybrid;
		}
		// config <= 31
		return PacketMode::CeltOnly;
	}
	pub fn get_bandwidth(&self) -> Bandwidth {
		// See "Table 2: TOC Byte Configuration Parameters"
		let config = self.0 & 63;
		if config <= 3 {
			return Bandwidth::NarrowBand;
		}
		if config <= 7 {
			return Bandwidth::MediumBand;
		}
		if config <= 11 {
			return Bandwidth::Wideband;
		}
		if config <= 13 {
			return Bandwidth::SuperWideband;
		}
		if config <= 15 {
			return Bandwidth::Fullband;
		}
		if config <= 19 {
			return Bandwidth::NarrowBand;
		}
		if config <= 23 {
			return Bandwidth::MediumBand;
		}
		if config <= 27 {
			return Bandwidth::Wideband;
		}
		// config <= 31
		return Bandwidth::Fullband;
	}
	pub fn get_frame_sizes(&self) -> FrameSize {
		// See "Table 2: TOC Byte Configuration Parameters"
		use self::FrameSize::*;
		let config = self.0 & 63;
		if config <= 11 {
			let fs_part = config & 3;
			return match fs_part {
				0 => Ms10,
				1 => Ms20,
				2 => Ms40,
				3 => Ms60,
				_ => unreachable!(),
			};
		}
		if config <= 15 {
			let fs_part = config & 1;
			if fs_part == 0 {
				return FrameSize::Ms10;
			} else {
				return FrameSize::Ms20;
			}
		}
		// config <= 31
		let fs_part = config & 3;
		return match fs_part {
			0 => Ms2p5,
			1 => Ms5,
			2 => Ms10,
			3 => Ms20,
			_ => unreachable!(),
		};
	}
	pub fn get_stereo(&self) -> bool {
		return (self.0 & 32) != 0;
	}
	pub fn get_frame_count_code(&self) -> u8 {
		return (self.0 & 192) >> 6;
	}
}
