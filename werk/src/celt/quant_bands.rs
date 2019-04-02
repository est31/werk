#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::f64::consts::LOG2_E;
use std::os::raw::{c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ulong, c_void};

extern "C" {
	#[no_mangle]
	fn log(_: c_double) -> c_double;
	#[no_mangle]
	fn floor(_: c_double) -> c_double;
	/*Returns the number of bits "used" by the encoded or decoded symbols so far.
	This same number can be computed in either the encoder or the decoder, and is
	 suitable for making coding decisions.
	Return: The number of bits scaled by 2**BITRES.
			This will always be slightly larger than the exact value (e.g., all
			 rounding error is in the positive direction).*/
	#[no_mangle]
	fn ec_tell_frac(_this: *mut ec_ctx) -> opus_uint32;
	/* Encode a bit that has a 1/(1<<_logp) probability of being a one */
	#[no_mangle]
	fn ec_enc_bit_logp(_this: *mut ec_enc, _val: c_int, _logp: c_uint);
	/*Encodes a symbol given an "inverse" CDF table.
	_s:    The index of the symbol to encode.
	_icdf: The "inverse" CDF, such that symbol _s falls in the range
			[_s>0?ft-_icdf[_s-1]:0,ft-_icdf[_s]), where ft=1<<_ftb.
		   The values must be monotonically non-increasing, and the last value
			must be 0.
	_ftb: The number of bits of precision in the cumulative distribution.*/
	#[no_mangle]
	fn ec_enc_icdf(_this: *mut ec_enc, _s: c_int, _icdf: *const c_uchar, _ftb: c_uint);
	/*Encodes a sequence of raw bits in the stream.
	_fl:  The bits to encode.
	_ftb: The number of bits to encode.
		  This must be between 1 and 25, inclusive.*/
	#[no_mangle]
	fn ec_enc_bits(_this: *mut ec_enc, _fl: opus_uint32, _ftb: c_uint);
	/* Decode a bit that has a 1/(1<<_logp) probability of being a one */
	#[no_mangle]
	fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: c_uint) -> c_int;
	/*Decodes a symbol given an "inverse" CDF table.
	No call to ec_dec_update() is necessary after this call.
	_icdf: The "inverse" CDF, such that symbol s falls in the range
			[s>0?ft-_icdf[s-1]:0,ft-_icdf[s]), where ft=1<<_ftb.
		   The values must be monotonically non-increasing, and the last value
			must be 0.
	_ftb: The number of bits of precision in the cumulative distribution.
	Return: The decoded symbol s.*/
	#[no_mangle]
	fn ec_dec_icdf(_this: *mut ec_dec, _icdf: *const c_uchar, _ftb: c_uint) -> c_int;
	/*Extracts a sequence of raw bits from the stream.
	The bits must have been encoded with ec_enc_bits().
	No call to ec_dec_update() is necessary after this call.
	_ftb: The number of bits to extract.
		  This must be between 0 and 25, inclusive.
	Return: The decoded bits.*/
	#[no_mangle]
	fn ec_dec_bits(_this: *mut ec_dec, _ftb: c_uint) -> opus_uint32;
	#[no_mangle]
	fn abs(_: c_int) -> c_int;
	#[no_mangle]
	fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;

	/* * Encode a value that is assumed to be the realisation of a
		Laplace-distributed random process
	 @param enc Entropy encoder state
	 @param value Value to encode
	 @param fs Probability of 0, multiplied by 32768
	 @param decay Probability of the value +/- 1, multiplied by 16384
	*/
	#[no_mangle]
	fn ec_laplace_encode(enc: *mut ec_enc, value: *mut c_int, fs: c_uint, decay: c_int);
	/* * Decode a value that is assumed to be the realisation of a
	   Laplace-distributed random process
	@param dec Entropy decoder state
	@param fs Probability of 0, multiplied by 32768
	@param decay Probability of the value +/- 1, multiplied by 16384
	@return Value decoded
	*/
	#[no_mangle]
	fn ec_laplace_decode(dec: *mut ec_dec, fs: c_uint, decay: c_int) -> c_int;
}

pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type opus_int16 = int16_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
pub type opus_val16 = c_float;
pub type opus_val32 = c_float;
pub type celt_ener = c_float;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpusCustomMode {
	pub Fs: opus_int32,
	pub overlap: c_int,
	pub nbEBands: c_int,
	pub effEBands: c_int,
	pub preemph: [opus_val16; 4],
	pub eBands: *const opus_int16,
	pub maxLM: c_int,
	pub nbShortMdcts: c_int,
	pub shortMdctSize: c_int,
	pub nbAllocVectors: c_int,
	pub allocVectors: *const c_uchar,
	pub logN: *const opus_int16,
	pub window: *const opus_val16,
	pub mdct: mdct_lookup,
	pub cache: PulseCache,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PulseCache {
	pub size: c_int,
	pub index: *const opus_int16,
	pub bits: *const c_uchar,
	pub caps: *const c_uchar,
}

/* This is a simple MDCT implementation that uses a N/4 complex FFT
   to do most of the work. It should be relatively straightforward to
   plug in pretty much and FFT here.

   This replaces the Vorbis FFT (and uses the exact same API), which
   was a bit too messy and that was ending up duplicating code
   (might as well use the same FFT everywhere).

   The algorithm is similar to (and inspired from) Fabrice Bellard's
   MDCT implementation in FFMPEG, but has differences in signs, ordering
   and scaling in many places.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdct_lookup {
	pub n: c_int,
	pub maxshift: c_int,
	pub kfft: [*const kiss_fft_state; 4],
	pub trig: *const c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kiss_fft_state {
	pub nfft: c_int,
	pub scale: opus_val16,
	pub shift: c_int,
	pub factors: [opus_int16; 16],
	pub bitrev: *const opus_int16,
	pub twiddles: *const kiss_twiddle_cpx,
	pub arch_fft: *mut arch_fft_state,
}
/* e.g. an fft of length 128 has 4 factors
as far as kissfft is concerned
4*4*4*2
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arch_fft_state {
	pub is_supported: c_int,
	pub priv_0: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kiss_twiddle_cpx {
	pub r: c_float,
	pub i: c_float,
}
/*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
larger type, you can speed up the decoder by using it here.*/
pub type ec_window = opus_uint32;
/*The number of bits to use for the range-coded part of unsigned integers.*/
/*The resolution of fractional-precision bit usage measurements, i.e.,
3 => 1/8th bits.*/
/*The entropy encoder/decoder context.
We use the same structure for both, so that common functions like ec_tell()
 can be used on either one.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_ctx {
	pub buf: *mut c_uchar,
	pub storage: opus_uint32,
	pub end_offs: opus_uint32,
	pub end_window: ec_window,
	pub nend_bits: c_int,
	pub nbits_total: c_int,
	pub offs: opus_uint32,
	pub rng: opus_uint32,
	pub val: opus_uint32,
	pub ext: opus_uint32,
	pub rem: c_int,
	pub error: c_int,
}
pub type ec_enc = ec_ctx;
pub type ec_dec = ec_ctx;
unsafe extern "C" fn ec_range_bytes(mut _this: *mut ec_ctx) -> opus_uint32 {
	(*_this).offs
}
unsafe extern "C" fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut c_uchar {
	(*_this).buf
}
/*Returns the number of bits "used" by the encoded or decoded symbols so far.
This same number can be computed in either the encoder or the decoder, and is
 suitable for making coding decisions.
Return: The number of bits.
		This will always be slightly larger than the exact value (e.g., all
		 rounding error is in the positive direction).*/
unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> c_int {
	(*_this).nbits_total
		- (::std::mem::size_of::<c_uint>() as c_ulong as c_int * 8i32
			- (*_this).rng.leading_zeros() as i32)
}

#[no_mangle]
pub static mut eMeans: [opus_val16; 25] = [
	6.437_5, 6.25, 5.75, 5.312_5, 5.062_5, 4.812_5, 4.5, 4.375, 4.875, 4.687_5, 4.562_5, 4.437_5,
	4.875, 4.625, 4.312_5, 4.500, 4.375, 4.625, 4.750, 4.437_5, 3.750, 3.750, 3.750, 3.750, 3.750,
];

#[no_mangle]
pub unsafe extern "C" fn amp2Log2(
	m: *const OpusCustomMode,
	effEnd: c_int,
	end: c_int,
	bandE: *mut celt_ener,
	bandLogE: *mut opus_val16,
	C: c_int,
) {
	let mut c: c_int;
	let mut i: c_int;
	c = 0i32;
	loop {
		i = 0i32;
		while i < effEnd {
			*bandLogE.offset((i + c * (*m).nbEBands) as isize) = (LOG2_E
				* log(f64::from(*bandE.offset((i + c * (*m).nbEBands) as isize))))
				as c_float - eMeans[i as usize];
			i += 1
		}
		i = effEnd;
		while i < end {
			*bandLogE.offset((c * (*m).nbEBands + i) as isize) = -14.0f32;
			i += 1
		}
		c += 1;
		if c >= C {
			break;
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn quant_coarse_energy(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	effEnd: c_int,
	eBands: *const opus_val16,
	oldEBands: *mut opus_val16,
	budget: opus_uint32,
	error: *mut opus_val16,
	enc: *mut ec_enc,
	C: c_int,
	LM: c_int,
	nbAvailableBytes: c_int,
	force_intra: c_int,
	delayedIntra: *mut opus_val32,
	mut two_pass: c_int,
	loss_rate: c_int,
	lfe: c_int,
) {
	let mut intra: c_int;
	let mut max_decay: opus_val16;
	let mut enc_start_state: ec_enc;
	let tell: opus_uint32;
	let mut badness1: c_int = 0i32;
	let intra_bias: opus_int32;
	let new_distortion: opus_val32;
	intra = (0 != force_intra
		|| 0 == two_pass
			&& *delayedIntra > (2i32 * C * (end - start)) as c_float
			&& nbAvailableBytes > (end - start) * C) as c_int;
	intra_bias = (budget as c_float * *delayedIntra * loss_rate as c_float
		/ (C * 512i32) as c_float) as opus_int32;
	new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
	tell = ec_tell(enc) as opus_uint32;
	if tell.wrapping_add(3i32 as c_uint) > budget {
		intra = 0i32;
		two_pass = intra
	}
	max_decay = 16.0f32;
	if end - start > 10i32 {
		max_decay = if max_decay < 0.125f32 * nbAvailableBytes as c_float {
			max_decay
		} else {
			0.125f32 * nbAvailableBytes as c_float
		}
	}
	if 0 != lfe {
		max_decay = 3.0f32
	}
	enc_start_state = *enc;
	let vla = (C * (*m).nbEBands) as usize;
	let mut oldEBands_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
	let vla_0 = (C * (*m).nbEBands) as usize;
	let mut error_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
	memcpy(
		oldEBands_intra.as_mut_ptr() as *mut c_void,
		oldEBands as *const c_void,
		((C * (*m).nbEBands) as c_ulong)
			.wrapping_mul(::std::mem::size_of::<opus_val16>() as c_ulong),
	);
	if 0 != two_pass || 0 != intra {
		badness1 = quant_coarse_energy_impl(
			m,
			start,
			end,
			eBands,
			oldEBands_intra.as_mut_ptr(),
			budget as opus_int32,
			tell as opus_int32,
			e_prob_model[LM as usize][1usize].as_ptr(),
			error_intra.as_mut_ptr(),
			enc,
			C,
			LM,
			1i32,
			max_decay,
			lfe,
		)
	}
	if 0 == intra {
		let intra_buf: *mut c_uchar;
		let mut enc_intra_state: ec_enc;
		let tell_intra: opus_int32;
		let nstart_bytes: opus_uint32;
		let nintra_bytes: opus_uint32;
		let mut save_bytes: opus_uint32;
		let badness2: c_int;
		tell_intra = ec_tell_frac(enc) as opus_int32;
		enc_intra_state = *enc;
		nstart_bytes = ec_range_bytes(&mut enc_start_state);
		nintra_bytes = ec_range_bytes(&mut enc_intra_state);
		intra_buf = ec_get_buffer(&mut enc_intra_state).offset(nstart_bytes as isize);
		save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
		if save_bytes == 0i32 as c_uint {
			save_bytes = 1i32 as opus_uint32
		}
		let vla_1 = save_bytes as usize;
		let mut intra_bits: Vec<c_uchar> = ::std::vec::from_elem(0, vla_1);
		memcpy(
			intra_bits.as_mut_ptr() as *mut c_void,
			intra_buf as *const c_void,
			u64::from(nintra_bytes.wrapping_sub(nstart_bytes))
				.wrapping_mul(::std::mem::size_of::<c_uchar>() as c_ulong),
		);
		*enc = enc_start_state;
		badness2 = quant_coarse_energy_impl(
			m,
			start,
			end,
			eBands,
			oldEBands,
			budget as opus_int32,
			tell as opus_int32,
			e_prob_model[LM as usize][intra as usize].as_ptr(),
			error,
			enc,
			C,
			LM,
			0i32,
			max_decay,
			lfe,
		);
		if 0 != two_pass
			&& (badness1 < badness2
				|| badness1 == badness2
					&& ec_tell_frac(enc) as opus_int32 + intra_bias > tell_intra)
		{
			*enc = enc_intra_state;
			memcpy(
				intra_buf as *mut c_void,
				intra_bits.as_mut_ptr() as *const c_void,
				u64::from(nintra_bytes.wrapping_sub(nstart_bytes))
					.wrapping_mul(::std::mem::size_of::<c_uchar>() as c_ulong),
			);
			memcpy(
				oldEBands as *mut c_void,
				oldEBands_intra.as_mut_ptr() as *const c_void,
				((C * (*m).nbEBands) as c_ulong)
					.wrapping_mul(::std::mem::size_of::<opus_val16>() as c_ulong),
			);
			memcpy(
				error as *mut c_void,
				error_intra.as_mut_ptr() as *const c_void,
				((C * (*m).nbEBands) as c_ulong)
					.wrapping_mul(::std::mem::size_of::<opus_val16>() as c_ulong),
			);
			intra = 1i32
		}
	} else {
		memcpy(
			oldEBands as *mut c_void,
			oldEBands_intra.as_mut_ptr() as *const c_void,
			((C * (*m).nbEBands) as c_ulong)
				.wrapping_mul(::std::mem::size_of::<opus_val16>() as c_ulong),
		);
		memcpy(
			error as *mut c_void,
			error_intra.as_mut_ptr() as *const c_void,
			((C * (*m).nbEBands) as c_ulong)
				.wrapping_mul(::std::mem::size_of::<opus_val16>() as c_ulong),
		);
	}
	if 0 != intra {
		*delayedIntra = new_distortion
	} else {
		*delayedIntra =
			pred_coef[LM as usize] * pred_coef[LM as usize] * *delayedIntra + new_distortion
	};
}

/* Mean energy in each band quantized in Q4 and converted back to float */
/* prediction coefficients: 0.9, 0.8, 0.65, 0.5 */
static mut pred_coef: [opus_val16; 4] = [
	(29440i32 as c_double / 32768.0f64) as opus_val16,
	(26112i32 as c_double / 32768.0f64) as opus_val16,
	(21248i32 as c_double / 32768.0f64) as opus_val16,
	(16384i32 as c_double / 32768.0f64) as opus_val16,
];
/*Parameters of the Laplace-like probability models used for the coarse energy.
There is one pair of parameters for each frame size, prediction type
 (inter/intra), and band number.
The first number of each pair is the probability of 0, and the second is the
 decay rate, both in Q8 precision.*/
static mut e_prob_model: [[[c_uchar; 42]; 2]; 4] = [
	[
		[
			72, 127, 65, 129, 66, 128, 65, 128, 64, 128, 62, 128, 64, 128, 64, 128, 92, 78, 92, 79,
			92, 78, 90, 79, 116, 41, 115, 40, 114, 40, 132, 26, 132, 26, 145, 17, 161, 12, 176, 10,
			177, 11,
		],
		[
			24, 179, 48, 138, 54, 135, 54, 132, 53, 134, 56, 133, 55, 132, 55, 132, 61, 114, 70,
			96, 74, 88, 75, 88, 87, 74, 89, 66, 91, 67, 100, 59, 108, 50, 120, 40, 122, 37, 97, 43,
			78, 50,
		],
	],
	[
		[
			83, 78, 84, 81, 88, 75, 86, 74, 87, 71, 90, 73, 93, 74, 93, 74, 109, 40, 114, 36, 117,
			34, 117, 34, 143, 17, 145, 18, 146, 19, 162, 12, 165, 10, 178, 7, 189, 6, 190, 8, 177,
			9,
		],
		[
			23, 178, 54, 115, 63, 102, 66, 98, 69, 99, 74, 89, 71, 91, 73, 91, 78, 89, 86, 80, 92,
			66, 93, 64, 102, 59, 103, 60, 104, 60, 117, 52, 123, 44, 138, 35, 133, 31, 97, 38, 77,
			45,
		],
	],
	[
		[
			61, 90, 93, 60, 105, 42, 107, 41, 110, 45, 116, 38, 113, 38, 112, 38, 124, 26, 132, 27,
			136, 19, 140, 20, 155, 14, 159, 16, 158, 18, 170, 13, 177, 10, 187, 8, 192, 6, 175, 9,
			159, 10,
		],
		[
			21, 178, 59, 110, 71, 86, 75, 85, 84, 83, 91, 66, 88, 73, 87, 72, 92, 75, 98, 72, 105,
			58, 107, 54, 115, 52, 114, 55, 112, 56, 129, 51, 132, 40, 150, 33, 140, 29, 98, 35, 77,
			42,
		],
	],
	[
		[
			42, 121, 96, 66, 108, 43, 111, 40, 117, 44, 123, 32, 120, 36, 119, 33, 127, 33, 134,
			34, 139, 21, 147, 23, 152, 20, 158, 25, 154, 26, 166, 21, 173, 16, 184, 13, 184, 10,
			150, 13, 139, 15,
		],
		[
			22, 178, 63, 114, 74, 82, 84, 83, 92, 82, 103, 62, 96, 72, 96, 67, 101, 73, 107, 72,
			113, 55, 118, 52, 125, 52, 118, 52, 117, 55, 135, 49, 137, 39, 157, 32, 145, 29, 97,
			33, 77, 40,
		],
	],
];

unsafe extern "C" fn quant_coarse_energy_impl(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	eBands: *const opus_val16,
	oldEBands: *mut opus_val16,
	budget: opus_int32,
	mut tell: opus_int32,
	prob_model: *const c_uchar,
	error: *mut opus_val16,
	enc: *mut ec_enc,
	C: c_int,
	LM: c_int,
	intra: c_int,
	max_decay: opus_val16,
	lfe: c_int,
) -> c_int {
	let mut i: c_int;
	let mut c: c_int;
	let mut badness: c_int = 0i32;
	let mut prev: [opus_val32; 2] = [0i32 as opus_val32, 0i32 as opus_val32];
	let coef: opus_val16;
	let beta: opus_val16;
	if tell + 3i32 <= budget {
		ec_enc_bit_logp(enc, intra, 3i32 as c_uint);
	}
	if 0 != intra {
		coef = 0i32 as opus_val16;
		beta = beta_intra
	} else {
		beta = beta_coef[LM as usize];
		coef = pred_coef[LM as usize]
	}
	i = start;
	while i < end {
		c = 0i32;
		loop {
			let bits_left: c_int;
			let mut qi: c_int;
			let qi0: c_int;
			let q: opus_val32;
			let x: opus_val16;
			let f: opus_val32;
			let tmp: opus_val32;
			let oldE: opus_val16;
			let decay_bound: opus_val16;
			x = *eBands.offset((i + c * (*m).nbEBands) as isize);
			oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
				-9.0f32
			} else {
				*oldEBands.offset((i + c * (*m).nbEBands) as isize)
			};
			f = x - coef * oldE - prev[c as usize];
			qi = floor(f64::from(0.5f32 + f)) as c_int;
			decay_bound = if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
				-28.0f32
			} else {
				*oldEBands.offset((i + c * (*m).nbEBands) as isize)
			} - max_decay;
			if qi < 0i32 && x < decay_bound {
				qi += (decay_bound - x) as c_int;
				if qi > 0i32 {
					qi = 0i32
				}
			}
			qi0 = qi;
			tell = ec_tell(enc);
			bits_left = budget - tell - 3i32 * C * (end - i);
			if i != start && bits_left < 30i32 {
				if bits_left < 24i32 {
					qi = if 1i32 < qi { 1i32 } else { qi }
				}
				if bits_left < 16i32 {
					qi = if -1i32 > qi { -1i32 } else { qi }
				}
			}
			if 0 != lfe && i >= 2i32 {
				qi = if qi < 0i32 { qi } else { 0i32 }
			}
			if budget - tell >= 15i32 {
				let pi: c_int;
				pi = 2i32 * if i < 20i32 { i } else { 20i32 };
				ec_laplace_encode(
					enc,
					&mut qi,
					(i32::from(*prob_model.offset(pi as isize)) << 7i32) as c_uint,
					i32::from(*prob_model.offset((pi + 1i32) as isize)) << 6i32,
				);
			} else if budget - tell >= 2i32 {
				qi = if -1i32 > if qi < 1i32 { qi } else { 1i32 } {
					-1i32
				} else if qi < 1i32 {
					qi
				} else {
					1i32
				};
				ec_enc_icdf(
					enc,
					(2i32 * qi) ^ -((qi < 0i32) as c_int),
					small_energy_icdf.as_ptr(),
					2i32 as c_uint,
				);
			} else if budget - tell >= 1i32 {
				qi = if 0i32 < qi { 0i32 } else { qi };
				ec_enc_bit_logp(enc, -qi, 1i32 as c_uint);
			} else {
				qi = -1i32
			}
			*error.offset((i + c * (*m).nbEBands) as isize) = f - qi as c_float;
			badness += abs(qi0 - qi);
			q = qi as opus_val32;
			tmp = coef * oldE + prev[c as usize] + q;
			*oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
			prev[c as usize] = prev[c as usize] + q - beta * q;
			c += 1;
			if c >= C {
				break;
			}
		}
		i += 1
	}
	if 0 != lfe {
		0i32
	} else {
		badness
	}
}

static mut small_energy_icdf: [c_uchar; 3] = [2i32 as c_uchar, 1i32 as c_uchar, 0i32 as c_uchar];
static mut beta_coef: [opus_val16; 4] = [
	(30147i32 as c_double / 32768.0f64) as opus_val16,
	(22282i32 as c_double / 32768.0f64) as opus_val16,
	(12124i32 as c_double / 32768.0f64) as opus_val16,
	(6554i32 as c_double / 32768.0f64) as opus_val16,
];
static mut beta_intra: opus_val16 = (4915i32 as c_double / 32768.0f64) as opus_val16;

unsafe extern "C" fn loss_distortion(
	eBands: *const opus_val16,
	old_ebands: *mut opus_val16,
	start: c_int,
	end: c_int,
	len: c_int,
	C: c_int,
) -> opus_val32 {
	let mut c: c_int;
	let mut i: c_int;
	let mut dist: opus_val32 = 0i32 as opus_val32;
	c = 0i32;
	loop {
		i = start;
		while i < end {
			let d: opus_val16 =
				*eBands.offset((i + c * len) as isize) - *old_ebands.offset((i + c * len) as isize);
			dist += d * d;
			i += 1
		}
		c += 1;
		if c >= C {
			break;
		}
	}
	if (200i32 as c_float) < dist {
		200i32 as c_float
	} else {
		dist
	}
}

#[no_mangle]
pub unsafe extern "C" fn quant_fine_energy(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	old_ebands: *mut opus_val16,
	error: *mut opus_val16,
	fine_quant: *mut c_int,
	enc: *mut ec_enc,
	C: c_int,
) {
	let mut i: c_int;
	let mut c: c_int;
	i = start;
	while i < end {
		let frac: opus_int16 = (1i32 << *fine_quant.offset(i as isize)) as opus_int16;
		if *fine_quant.offset(i as isize) > 0i32 {
			c = 0i32;
			loop {
				let mut q2: c_int;
				let offset: opus_val16;
				q2 = floor(f64::from(
					(*error.offset((i + c * (*m).nbEBands) as isize) + 0.5f32)
						* i32::from(frac) as c_float,
				)) as c_int;
				if q2 > i32::from(frac) - 1i32 {
					q2 = i32::from(frac) - 1i32
				}
				if q2 < 0i32 {
					q2 = 0i32
				}
				ec_enc_bits(
					enc,
					q2 as opus_uint32,
					*fine_quant.offset(i as isize) as c_uint,
				);
				offset = (q2 as c_float + 0.5f32)
					* (1i32 << (14i32 - *fine_quant.offset(i as isize))) as c_float
					* (1.0f32 / 16384i32 as c_float)
					- 0.5f32;
				let fresh0 = &mut (*old_ebands.offset((i + c * (*m).nbEBands) as isize));
				*fresh0 += offset;
				let fresh1 = &mut (*error.offset((i + c * (*m).nbEBands) as isize));
				*fresh1 -= offset;
				c += 1;
				if c >= C {
					break;
				}
			}
		}
		i += 1
	}
}

#[no_mangle]
pub unsafe extern "C" fn quant_energy_finalise(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	old_ebands: *mut opus_val16,
	error: *mut opus_val16,
	fine_quant: *mut c_int,
	fine_priority: *mut c_int,
	mut bits_left: c_int,
	enc: *mut ec_enc,
	C: c_int,
) {
	let mut i: c_int;
	let mut prio: c_int;
	let mut c: c_int;
	prio = 0i32;
	while prio < 2i32 {
		i = start;
		while i < end && bits_left >= C {
			if !(*fine_quant.offset(i as isize) >= 8i32
				|| *fine_priority.offset(i as isize) != prio)
			{
				c = 0i32;
				loop {
					let q2: c_int;
					let offset: opus_val16;
					q2 = if *error.offset((i + c * (*m).nbEBands) as isize) < 0i32 as c_float {
						0i32
					} else {
						1i32
					};
					ec_enc_bits(enc, q2 as opus_uint32, 1i32 as c_uint);
					offset = (q2 as c_float - 0.5f32)
						* (1i32 << (14i32 - *fine_quant.offset(i as isize) - 1i32)) as c_float
						* (1.0f32 / 16384i32 as c_float);
					let fresh2 = &mut (*old_ebands.offset((i + c * (*m).nbEBands) as isize));
					*fresh2 += offset;
					let fresh3 = &mut (*error.offset((i + c * (*m).nbEBands) as isize));
					*fresh3 -= offset;
					bits_left -= 1;
					c += 1;
					if c >= C {
						break;
					}
				}
			}
			i += 1
		}
		prio += 1
	}
}

#[no_mangle]
pub unsafe extern "C" fn unquant_coarse_energy(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	old_ebands: *mut opus_val16,
	intra: c_int,
	dec: *mut ec_dec,
	C: c_int,
	LM: c_int,
) {
	let prob_model: *const c_uchar = e_prob_model[LM as usize][intra as usize].as_ptr();
	let mut i: c_int;
	let mut c: c_int;
	let mut prev: [opus_val32; 2] = [0i32 as opus_val32, 0i32 as opus_val32];
	let coef: opus_val16;
	let beta: opus_val16;
	let budget: opus_int32;
	let mut tell: opus_int32;
	if 0 != intra {
		coef = 0i32 as opus_val16;
		beta = beta_intra
	} else {
		beta = beta_coef[LM as usize];
		coef = pred_coef[LM as usize]
	}
	budget = (*dec).storage.wrapping_mul(8i32 as c_uint) as opus_int32;
	i = start;
	while i < end {
		c = 0i32;
		loop {
			let mut qi: c_int;
			let q: opus_val32;
			let tmp: opus_val32;
			tell = ec_tell(dec);
			if budget - tell >= 15i32 {
				let pi: c_int = 2i32 * if i < 20i32 { i } else { 20i32 };
				qi = ec_laplace_decode(
					dec,
					(i32::from(*prob_model.offset(pi as isize)) << 7i32) as c_uint,
					i32::from(*prob_model.offset((pi + 1i32) as isize)) << 6i32,
				)
			} else if budget - tell >= 2i32 {
				qi = ec_dec_icdf(dec, small_energy_icdf.as_ptr(), 2i32 as c_uint);
				qi = qi >> 1i32 ^ -(qi & 1i32)
			} else if budget - tell >= 1i32 {
				qi = -ec_dec_bit_logp(dec, 1i32 as c_uint)
			} else {
				qi = -1i32
			}
			q = qi as opus_val32;
			*old_ebands.offset((i + c * (*m).nbEBands) as isize) =
				if -9.0f32 > *old_ebands.offset((i + c * (*m).nbEBands) as isize) {
					-9.0f32
				} else {
					*old_ebands.offset((i + c * (*m).nbEBands) as isize)
				};
			tmp =
				coef * *old_ebands.offset((i + c * (*m).nbEBands) as isize) + prev[c as usize] + q;
			*old_ebands.offset((i + c * (*m).nbEBands) as isize) = tmp;
			prev[c as usize] = prev[c as usize] + q - beta * q;
			c += 1;
			if c >= C {
				break;
			}
		}
		i += 1
	}
}

#[no_mangle]
pub unsafe extern "C" fn unquant_fine_energy(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	old_ebands: *mut opus_val16,
	fine_quant: *mut c_int,
	dec: *mut ec_dec,
	C: c_int,
) {
	let mut i: c_int;
	let mut c: c_int;
	i = start;
	while i < end {
		if *fine_quant.offset(i as isize) > 0i32 {
			c = 0i32;
			loop {
				let q2: c_int;
				let offset: opus_val16;
				q2 = ec_dec_bits(dec, *fine_quant.offset(i as isize) as c_uint) as c_int;
				offset = (q2 as c_float + 0.5f32)
					* (1i32 << (14i32 - *fine_quant.offset(i as isize))) as c_float
					* (1.0f32 / 16384i32 as c_float)
					- 0.5f32;
				let fresh4 = &mut (*old_ebands.offset((i + c * (*m).nbEBands) as isize));
				*fresh4 += offset;
				c += 1;
				if c >= C {
					break;
				}
			}
		}
		i += 1
	}
}

#[no_mangle]
pub unsafe extern "C" fn unquant_energy_finalise(
	m: *const OpusCustomMode,
	start: c_int,
	end: c_int,
	old_ebands: *mut opus_val16,
	fine_quant: *mut c_int,
	fine_priority: *mut c_int,
	mut bits_left: c_int,
	dec: *mut ec_dec,
	C: c_int,
) {
	let mut i: c_int;
	let mut prio: c_int;
	let mut c: c_int;
	prio = 0i32;
	while prio < 2i32 {
		i = start;
		while i < end && bits_left >= C {
			if !(*fine_quant.offset(i as isize) >= 8i32
				|| *fine_priority.offset(i as isize) != prio)
			{
				c = 0i32;
				loop {
					let q2: c_int;
					let offset: opus_val16;
					q2 = ec_dec_bits(dec, 1i32 as c_uint) as c_int;
					offset = (q2 as c_float - 0.5f32)
						* (1i32 << (14i32 - *fine_quant.offset(i as isize) - 1i32)) as c_float
						* (1.0f32 / 16384i32 as c_float);
					let fresh5 = &mut (*old_ebands.offset((i + c * (*m).nbEBands) as isize));
					*fresh5 += offset;
					bits_left -= 1;
					c += 1;
					if c >= C {
						break;
					}
				}
			}
			i += 1
		}
		prio += 1
	}
}
