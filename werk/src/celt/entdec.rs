// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

/*!
A range decoder.
*/

// TODO: put the text below into the doc comment.
// For this we need ways of to how to represent the formatting the text does
/*
This is an entropy decoder based upon \cite{Mar79}, which is itself a
 rediscovery of the FIFO arithmetic code introduced by \cite{Pas76}.
It is very similar to arithmetic encoding, except that encoding is done with
 digits in any base, instead of with bits, and so it is faster when using
 larger bases (i.e.: a byte).
The author claims an average waste of $\frac{1}{2}\log_b(2b)$ bits, where $b$
 is the base, longer than the theoretical optimum, but to my knowledge there
 is no published justification for this claim.
This only seems true when using near-infinite precision arithmetic so that
 the process is carried out with no rounding errors.

An excellent description of implementation details is available at
 http://www.arturocampos.com/ac_range.html
A recent work \cite{MNW98} which proposes several changes to arithmetic
 encoding for efficiency actually re-discovers many of the principles
 behind range encoding, and presents a good theoretical analysis of them.

End of stream is handled by writing out the smallest number of bits that
 ensures that the stream will be correctly decoded regardless of the value of
 any subsequent bits.
ec_tell() can be used to determine how many bits were needed to decode
 all the symbols thus far; other data can be packed in the remaining bits of
 the input buffer.
@PHDTHESIS{Pas76,
  author="Richard Clark Pasco",
  title="Source coding algorithms for fast data compression",
  school="Dept. of Electrical Engineering, Stanford University",
  address="Stanford, CA",
  month=May,
  year=1976
}
@INPROCEEDINGS{Mar79,
 author="Martin, G.N.N.",
 title="Range encoding: an algorithm for removing redundancy from a digitised
  message",
 booktitle="Video & Data Recording Conference",
 year=1979,
 address="Southampton",
 month=Jul
}
@ARTICLE{MNW98,
 author="Alistair Moffat and Radford Neal and Ian H. Witten",
 title="Arithmetic Coding Revisited",
 journal="{ACM} Transactions on Information Systems",
 year=1998,
 volume=16,
 number=3,
 pages="256--294",
 month=Jul,
 URL="http://www.stanford.edu/class/ee398a/handouts/papers/Moffat98ArithmCoding.pdf"
}
*/

use super::entcode::*;
use super::mathops::ilog;
use std::cmp;
use std::os::raw::*;
use std::ptr;

pub type ec_dec = ec_ctx;

impl ec_dec {
	fn read_byte(&mut self) -> u8 {
		if self.offs < self.storage {
			let res = self.get_buf()[self.offs as usize];
			self.offs += 1;
			res
		} else {
			0
		}
	}

	fn read_byte_from_end(&mut self) -> u8 {
		if self.end_offs < self.storage {
			self.end_offs += 1;
			self.get_buf()[(self.storage - self.end_offs) as usize]
		} else {
			0
		}
	}

	fn dec_normalize(&mut self) {
		while self.rng <= EC_CODE_BOT {
			self.nbits_total += EC_SYM_BITS as i32;
			self.rng <<= EC_SYM_BITS;
			// Use up the remaining bits from our last symbol.
			let mut sym = self.rem;
			// Read the next value from the input.
			self.rem = i32::from(self.read_byte());
			// Take the rest of the bits we need from this new symbol.
			sym = (sym << EC_SYM_BITS | self.rem) >> (EC_SYM_BITS - EC_CODE_EXTRA);
			// And subtract them from val, capped to be less than EC_CODE_TOP.
			self.val = ((self.val << EC_SYM_BITS) + (EC_SYM_MAX as u32 & !(sym as u32)))
				& (EC_CODE_TOP - 1);
		}
	}
}

#[no_mangle]
/// Initializes the decoder.
pub extern "C" fn ec_dec_init(this: &mut ec_dec, buf: *mut c_uchar, storage: u32) {
	this.buf = buf;
	this.storage = storage;
	this.end_offs = 0;
	this.end_window = 0;
	this.nend_bits = 0;
	// This is the offset from which ec_tell() will subtract partial bits.
	// The final value after the ec_dec_normalize() call will be the same as in
	// the encoder, but we have to compensate for the bits that are added
	// there.
	this.nbits_total =
		EC_CODE_BITS + 1 - ((EC_CODE_BITS - EC_CODE_EXTRA) / EC_SYM_BITS) * EC_SYM_BITS;
	this.offs = 0;
	this.rng = 1 << EC_CODE_EXTRA;
	this.rem = i32::from(this.read_byte());
	this.val = this.rng - 1 - (this.rem as u32 >> (EC_SYM_BITS - EC_CODE_EXTRA));
	this.error = 0;
	// Normalize the interval
	this.dec_normalize();
}

#[no_mangle]
/**
Calculates the cumulative frequency for the next symbol.

This can then be fed into the probability model to determine what that
symbol is, and the additional frequency information required to advance to
the next symbol.
This function cannot be called more than once without a corresponding call to
ec_dec_update(), or decoding will not proceed correctly.

* `ft`: The total frequency of the symbols in the alphabet the next symbol was
	encoded with.

Return: A cumulative frequency representing the encoded symbol.
	If the cumulative frequency of all the symbols before the one that
	was encoded was fl, and the cumulative frequency of all the symbols
	up to and including the one encoded is fh, then the returned value
	will fall in the range [fl,fh).
*/
pub extern "C" fn ec_decode(this: &mut ec_dec, ft: c_uint) -> c_uint {
	this.ext = celt_udiv(this.rng, ft);
	let s = this.val / this.ext;
	// libopus uses a macro called "EC_MINI" here,
	// that has an implementation that runs fast on old (<4.x) gcc
	// compilers. We just use std and hope that its either similarly
	// optimized, or that llvm does the optimization for us.
	ft - cmp::min(s + 1, ft)
}

#[no_mangle]
/// Equivalent to ec_decode() with `ft==1<<bits`.
pub extern "C" fn ec_decode_bin(this: &mut ec_dec, bits: c_uint) -> c_uint {
	this.ext = this.rng >> bits;
	let s = this.val / this.ext;
	(1 << bits) - cmp::min(s + 1, 1 << bits)
}

#[no_mangle]
/**
Advance the decoder past the next symbol using the frequency information the
symbol was encoded with.

Exactly one call to ec_decode() must have been made so that all necessary
intermediate calculations are performed.

* `fl`:  The cumulative frequency of all symbols that come before the symbol
	decoded.
* `fh`:  The cumulative frequency of all symbols up to and including the symbol
	decoded. Together with _fl, this defines the range [_fl,_fh) in which the
	value returned above must fall.
* `ft`:  The total frequency of the symbols in the alphabet the symbol decoded
	was encoded in. This must be the same as passed to the preceding call
	to ec_decode().
*/
pub extern "C" fn ec_dec_update(this: &mut ec_dec, fl: c_uint, fh: c_uint, ft: c_uint) {
	let s = this.ext * (ft - fh);
	this.val -= s;
	this.rng = if fl > 0 {
		this.ext * (fh - fl)
	} else {
		this.rng - s
	};
	this.dec_normalize();
}

#[no_mangle]
/// Decode a bit that has a 1/(1<<_logp) probability of being a one.
pub extern "C" fn ec_dec_bit_logp(this: &mut ec_dec, logp: c_uint) -> c_int {
	let r = this.rng;
	let d = this.val;
	let s = r >> logp;
	let ret = d < s;
	if !ret {
		this.val = d - s;
	}
	this.rng = if ret { s } else { r - s };
	this.dec_normalize();
	ret as c_int
}

#[no_mangle]
/**
Decodes a symbol given an "inverse" CDF table.

No call to ec_dec_update() is necessary after this call.

* `icdf`: The "inverse" CDF, such that symbol s falls in the range
	`[s>0?ft-_icdf[s-1]:0,ft-_icdf[s])`, where `ft=1<<_ftb`.
	The values must be monotonically non-increasing, and the last value
	must be `0`.
* `ftb`: The number of bits of precision in the cumulative distribution.

Return: The decoded symbol `s`.
*/
pub extern "C" fn ec_dec_icdf(this: &mut ec_dec, icdf: *const u8, ftb: c_uint) -> c_int {
	let mut s = this.rng;
	let d = this.val;
	let r = s >> ftb;
	let mut ret = -1;
	let mut t;
	loop {
		t = s;
		ret += 1;
		unsafe {
			s = r * u32::from(ptr::read(icdf.offset(ret as isize)));
		}
		if d >= s {
			break;
		}
	}
	this.val = d - s;
	this.rng = t - s;
	this.dec_normalize();
	ret
}

#[no_mangle]
/**
Extracts a raw unsigned integer with a non-power-of-2 range from the stream.

The bits must have been encoded with ec_enc_uint().
No call to ec_dec_update() is necessary after this call.

* `ft`: The number of integers that can be decoded (one more than the max).
	This must be at least 2, and no more than `2**32-1`.

Return: The decoded bits.
*/
pub extern "C" fn ec_dec_uint(this: &mut ec_dec, mut ft: u32) -> u32 {
	// In order to optimize EC_ILOG(), it is undefined for the value 0.
	assert!(ft > 1);
	ft -= 1;
	let mut ftb = ilog(ft);
	if ftb > EC_UINT_BITS {
		ftb -= EC_UINT_BITS;
		let fta = (ft >> ftb) + 1;
		let s = ec_decode(this, fta);
		ec_dec_update(this, s, s + 1, fta);
		let t = (s as u32) << ftb | ec_dec_bits(this, ftb);
		if t <= ft {
			t
		} else {
			this.error = 1;
			ft
		}
	} else {
		ft += 1;
		let s = ec_decode(this, ft);
		ec_dec_update(this, s, s + 1, ft);
		s
	}
}

#[no_mangle]
/**
Extracts a sequence of raw bits from the stream.

The bits must have been encoded with `ec_enc_bits()`.
No call to `ec_dec_update()` is necessary after this call.

* `bits`: The number of bits to extract.
	This must be between 0 and 25, inclusive.

Return: The decoded bits.
*/
pub extern "C" fn ec_dec_bits(this: &mut ec_dec, bits: c_uint) -> u32 {
	let mut window = this.end_window;
	let mut available = this.nend_bits;
	if (available as c_uint) < bits {
		loop {
			window |= u32::from(this.read_byte_from_end()) << available;
			available += EC_SYM_BITS;
			if available > (EC_WINDOW_SIZE as i32) - EC_SYM_BITS {
				break;
			}
		}
	}
	let ret = window & ((1 << bits) - 1);
	window >>= bits;
	available -= bits as i32;
	this.end_window = window;
	this.nend_bits = available;
	this.nbits_total += bits as i32;
	ret
}
