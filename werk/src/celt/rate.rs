#![allow(clippy::cyclomatic_complexity)]
#![allow(non_snake_case)]

use std::os::raw::*;

extern "C" {
    #[no_mangle]
    fn ec_enc_bit_logp(_this: *mut ec_enc, _val: c_int, _logp: c_uint);
    /*Encodes a raw unsigned integer in the stream.
    _fl: The integer to encode.
    _ft: The number of integers that can be encoded (one more than the max).
         This must be at least 2, and no more than 2**32-1.*/
    #[no_mangle]
    fn ec_enc_uint(_this: *mut ec_enc, _fl: opus_uint32, _ft: opus_uint32);
    /* Decode a bit that has a 1/(1<<_logp) probability of being a one */
    #[no_mangle]
    fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: c_uint) -> c_int;
    /*Extracts a raw unsigned integer with a non-power-of-2 range from the stream.
    The bits must have been encoded with ec_enc_uint().
    No call to ec_dec_update() is necessary after this call.
    _ft: The number of integers that can be decoded (one more than the max).
         This must be at least 2, and no more than 2**32-1.
    Return: The decoded bits.*/
    #[no_mangle]
    fn ec_dec_uint(_this: *mut ec_dec, _ft: opus_uint32) -> opus_uint32;
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

/* * Mode definition (opaque)
@brief Mode definition
*/
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

/* Set this if opus_int64 is a native type of the CPU. */
/* Assume that all LP64 architectures have fast 64-bit types; also x86_64
(which can be ILP32 for x32) and Win64 (which is LLP64). */
/* FIXED_POINT */
pub type opus_val16 = c_float;
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

/* Tested exhaustively for all n and for 1<=d<=256 */
extern "C" fn celt_udiv(n: opus_uint32, d: opus_uint32) -> opus_uint32 {
    n.wrapping_div(d)
}

/* * Compute the pulse allocation, i.e. how many pulses will go in each
  * band.
 @param m mode
 @param offsets Requested increase or decrease in the number of bits for
                each band
 @param total Number of bands
 @param pulses Number of pulses per band (returned)
 @return Total number of bits allocated
*/
#[no_mangle]
pub unsafe extern "C" fn compute_allocation(
    m: *const OpusCustomMode,
    start: c_int,
    end: c_int,
    offsets: *const c_int,
    cap: *const c_int,
    alloc_trim: c_int,
    intensity: *mut c_int,
    dual_stereo: *mut c_int,
    mut total: opus_int32,
    balance: *mut opus_int32,
    pulses: *mut c_int,
    ebits: *mut c_int,
    fine_priority: *mut c_int,
    C: c_int,
    LM: c_int,
    ec: *mut ec_ctx,
    encode: c_int,
    prev: c_int,
    signalBandwidth: c_int,
) -> c_int {
    let mut lo: c_int;
    let mut hi: c_int;
    let len: c_int;
    let mut j: c_int;
    let codedBands: c_int;
    let mut skip_start: c_int;
    let skip_rsv: c_int;
    let mut intensity_rsv: c_int;
    let mut dual_stereo_rsv: c_int;
    total = if total > 0i32 { total } else { 0i32 };
    len = (*m).nbEBands;
    skip_start = start;
    skip_rsv = if total >= 1i32 << 3i32 {
        1i32 << 3i32
    } else {
        0i32
    };
    total -= skip_rsv;
    dual_stereo_rsv = 0i32;
    intensity_rsv = dual_stereo_rsv;
    if C == 2i32 {
        intensity_rsv = i32::from(LOG2_FRAC_TABLE[(end - start) as usize]);
        if intensity_rsv > total {
            intensity_rsv = 0i32
        } else {
            total -= intensity_rsv;
            dual_stereo_rsv = if total >= 1i32 << 3i32 {
                1i32 << 3i32
            } else {
                0i32
            };
            total -= dual_stereo_rsv
        }
    }
    let vla = len as usize;
    let mut bits1: Vec<c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = len as usize;
    let mut bits2: Vec<c_int> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = len as usize;
    let mut thresh: Vec<c_int> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = len as usize;
    let mut trim_offset: Vec<c_int> = ::std::vec::from_elem(0, vla_2);
    j = start;
    while j < end {
        *thresh.as_mut_ptr().offset(j as isize) = if C << 3i32
            > (3i32
                * (i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                    - i32::from(*(*m).eBands.offset(j as isize))))
                << LM
                << 3i32
                >> 4i32
        {
            C << 3i32
        } else {
            (3i32
                * (i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                    - i32::from(*(*m).eBands.offset(j as isize))))
                << LM
                << 3i32
                >> 4i32
        };
        *trim_offset.as_mut_ptr().offset(j as isize) = (C
            * (i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                - i32::from(*(*m).eBands.offset(j as isize)))
            * (alloc_trim - 5i32 - LM)
            * (end - j - 1i32)
            * (1i32 << (LM + 3i32)))
            >> 6i32;
        if (i32::from(*(*m).eBands.offset((j + 1i32) as isize))
            - i32::from(*(*m).eBands.offset(j as isize)))
            << LM
            == 1i32
        {
            *trim_offset.as_mut_ptr().offset(j as isize) -= C << 3i32
        }
        j += 1
    }
    lo = 1i32;
    hi = (*m).nbAllocVectors - 1i32;
    loop {
        let mut done: c_int = 0i32;
        let mut psum: c_int = 0i32;
        let mid: c_int = (lo + hi) >> 1i32;
        j = end;
        loop {
            let fresh0 = j;
            j -= 1;
            if fresh0 <= start {
                break;
            }
            let mut bitsj: c_int;
            let N: c_int = i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                - i32::from(*(*m).eBands.offset(j as isize));
            bitsj = (C * N * i32::from(*(*m).allocVectors.offset((mid * len + j) as isize))) << LM
                >> 2i32;
            if bitsj > 0i32 {
                bitsj = if 0i32 > bitsj + *trim_offset.as_mut_ptr().offset(j as isize) {
                    0i32
                } else {
                    bitsj + *trim_offset.as_mut_ptr().offset(j as isize)
                }
            }
            bitsj += *offsets.offset(j as isize);
            if bitsj >= *thresh.as_mut_ptr().offset(j as isize) || 0 != done {
                done = 1i32;
                psum += if bitsj < *cap.offset(j as isize) {
                    bitsj
                } else {
                    *cap.offset(j as isize)
                }
            } else if bitsj >= C << 3i32 {
                psum += C << 3i32
            }
        }
        if psum > total {
            hi = mid - 1i32
        } else {
            lo = mid + 1i32
        }
        if lo > hi {
            break;
        }
    }
    let fresh1 = lo;
    lo -= 1;
    hi = fresh1;
    j = start;
    while j < end {
        let mut bits1j: c_int;
        let mut bits2j: c_int;
        let N_0: c_int = i32::from(*(*m).eBands.offset((j + 1i32) as isize))
            - i32::from(*(*m).eBands.offset(j as isize));
        bits1j =
            (C * N_0 * i32::from(*(*m).allocVectors.offset((lo * len + j) as isize))) << LM >> 2i32;
        bits2j = if hi >= (*m).nbAllocVectors {
            *cap.offset(j as isize)
        } else {
            (C * N_0 * i32::from(*(*m).allocVectors.offset((hi * len + j) as isize))) << LM >> 2i32
        };
        if bits1j > 0i32 {
            bits1j = if 0i32 > bits1j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0i32
            } else {
                bits1j + *trim_offset.as_mut_ptr().offset(j as isize)
            }
        }
        if bits2j > 0i32 {
            bits2j = if 0i32 > bits2j + *trim_offset.as_mut_ptr().offset(j as isize) {
                0i32
            } else {
                bits2j + *trim_offset.as_mut_ptr().offset(j as isize)
            }
        }
        if lo > 0i32 {
            bits1j += *offsets.offset(j as isize)
        }
        bits2j += *offsets.offset(j as isize);
        if *offsets.offset(j as isize) > 0i32 {
            skip_start = j
        }
        bits2j = if 0i32 > bits2j - bits1j {
            0i32
        } else {
            bits2j - bits1j
        };
        *bits1.as_mut_ptr().offset(j as isize) = bits1j;
        *bits2.as_mut_ptr().offset(j as isize) = bits2j;
        j += 1
    }
    codedBands = interp_bits2pulses(
        m,
        start,
        end,
        skip_start,
        bits1.as_mut_ptr(),
        bits2.as_mut_ptr(),
        thresh.as_mut_ptr(),
        cap,
        total,
        balance,
        skip_rsv,
        intensity,
        intensity_rsv,
        dual_stereo,
        dual_stereo_rsv,
        pulses,
        ebits,
        fine_priority,
        C,
        LM,
        ec,
        encode,
        prev,
        signalBandwidth,
    );
    codedBands
}
/* CUSTOM_MODES */
unsafe extern "C" fn interp_bits2pulses(
    m: *const OpusCustomMode,
    start: c_int,
    end: c_int,
    skip_start: c_int,
    bits1: *const c_int,
    bits2: *const c_int,
    thresh: *const c_int,
    cap: *const c_int,
    mut total: opus_int32,
    mut _balance: *mut opus_int32,
    skip_rsv: c_int,
    intensity: *mut c_int,
    mut intensity_rsv: c_int,
    dual_stereo: *mut c_int,
    mut dual_stereo_rsv: c_int,
    bits: *mut c_int,
    ebits: *mut c_int,
    fine_priority: *mut c_int,
    C: c_int,
    LM: c_int,
    ec: *mut ec_ctx,
    encode: c_int,
    prev: c_int,
    signalBandwidth: c_int,
) -> c_int {
    let mut psum: opus_int32;
    let mut lo: c_int;
    let mut hi: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let logM: c_int;
    let stereo: c_int;
    let mut codedBands: c_int;
    let alloc_floor: c_int;
    let mut left: opus_int32;
    let mut percoeff: opus_int32;
    let mut done: c_int;
    let mut balance: opus_int32;
    alloc_floor = C << 3i32;
    stereo = (C > 1i32) as c_int;
    logM = LM << 3i32;
    lo = 0i32;
    hi = 1i32 << 6i32;
    i = 0i32;
    while i < 6i32 {
        let mid: c_int = (lo + hi) >> 1i32;
        psum = 0i32;
        done = 0i32;
        j = end;
        loop {
            let fresh2 = j;
            j -= 1;
            if fresh2 <= start {
                break;
            }
            let tmp: c_int =
                *bits1.offset(j as isize) + ((mid * *bits2.offset(j as isize)) >> 6i32);
            if tmp >= *thresh.offset(j as isize) || 0 != done {
                done = 1i32;
                psum += if tmp < *cap.offset(j as isize) {
                    tmp
                } else {
                    *cap.offset(j as isize)
                }
            } else if tmp >= alloc_floor {
                psum += alloc_floor
            }
        }
        if psum > total {
            hi = mid
        } else {
            lo = mid
        }
        i += 1
    }
    psum = 0i32;
    done = 0i32;
    j = end;
    loop {
        let fresh3 = j;
        j -= 1;
        if fresh3 <= start {
            break;
        }
        let mut tmp_0: c_int =
            *bits1.offset(j as isize) + ((lo * *bits2.offset(j as isize)) >> 6i32);
        if tmp_0 < *thresh.offset(j as isize) && 0 == done {
            if tmp_0 >= alloc_floor {
                tmp_0 = alloc_floor
            } else {
                tmp_0 = 0i32
            }
        } else {
            done = 1i32
        }
        tmp_0 = if tmp_0 < *cap.offset(j as isize) {
            tmp_0
        } else {
            *cap.offset(j as isize)
        };
        *bits.offset(j as isize) = tmp_0;
        psum += tmp_0
    }
    codedBands = end;
    loop {
        let band_width: c_int;
        let mut band_bits: c_int;
        let rem: c_int;
        j = codedBands - 1i32;
        /* Never skip the first band, nor a band that has been boosted by
         dynalloc.
        In the first case, we'd be coding a bit to signal we're going to waste
         all the other bits.
        In the second case, we'd be coding a bit to redistribute all the bits
         we just signaled should be cocentrated in this band. */
        if j <= skip_start {
            total += skip_rsv;
            break;
        } else {
            left = total - psum;
            percoeff = celt_udiv(
                left as opus_uint32,
                (i32::from(*(*m).eBands.offset(codedBands as isize))
                    - i32::from(*(*m).eBands.offset(start as isize)))
                    as opus_uint32,
            ) as opus_int32;
            left -= (i32::from(*(*m).eBands.offset(codedBands as isize))
                - i32::from(*(*m).eBands.offset(start as isize)))
                * percoeff;
            rem = if left
                - (i32::from(*(*m).eBands.offset(j as isize))
                    - i32::from(*(*m).eBands.offset(start as isize)))
                > 0i32
            {
                left - (i32::from(*(*m).eBands.offset(j as isize))
                    - i32::from(*(*m).eBands.offset(start as isize)))
            } else {
                0i32
            };
            band_width = i32::from(*(*m).eBands.offset(codedBands as isize))
                - i32::from(*(*m).eBands.offset(j as isize));
            band_bits = *bits.offset(j as isize) + percoeff * band_width + rem;
            /*Only code a skip decision if we're above the threshold for this band.
            Otherwise it is force-skipped.
            This ensures that we have enough bits to code the skip flag.*/
            if band_bits
                >= if *thresh.offset(j as isize) > alloc_floor + (1i32 << 3i32) {
                    *thresh.offset(j as isize)
                } else {
                    alloc_floor + (1i32 << 3i32)
                }
            {
                if 0 != encode {
                    /*This if() block is the only part of the allocation function that
                    is not a mandatory part of the bitstream: any bands we choose to
                    skip here must be explicitly signaled.*/
                    let depth_threshold: c_int;
                    if codedBands > 17i32 {
                        depth_threshold = if j < prev { 7i32 } else { 9i32 }
                    } else {
                        depth_threshold = 0i32
                    }
                    if codedBands <= start + 2i32
                        || band_bits > (depth_threshold * band_width) << LM << 3i32 >> 4i32
                            && j <= signalBandwidth
                    {
                        ec_enc_bit_logp(ec, 1i32, 1i32 as c_uint);
                        break;
                    } else {
                        ec_enc_bit_logp(ec, 0i32, 1i32 as c_uint);
                    }
                } else if 0 != ec_dec_bit_logp(ec, 1i32 as c_uint) {
                    break;
                }
                psum += 1i32 << 3i32;
                band_bits -= 1i32 << 3i32
            }
            psum -= *bits.offset(j as isize) + intensity_rsv;
            if intensity_rsv > 0i32 {
                intensity_rsv = i32::from(LOG2_FRAC_TABLE[(j - start) as usize])
            }
            psum += intensity_rsv;
            if band_bits >= alloc_floor {
                psum += alloc_floor;
                *bits.offset(j as isize) = alloc_floor
            } else {
                *bits.offset(j as isize) = 0i32
            }
            codedBands -= 1
        }
    }
    if codedBands <= start {
        eprintln!("Fatal (internal) error in rate.rs\n");
        panic!()
    }
    if intensity_rsv > 0i32 {
        if 0 != encode {
            *intensity = if *intensity < codedBands {
                *intensity
            } else {
                codedBands
            };
            ec_enc_uint(
                ec,
                (*intensity - start) as opus_uint32,
                (codedBands + 1i32 - start) as opus_uint32,
            );
        } else {
            *intensity = (start as c_uint)
                .wrapping_add(ec_dec_uint(ec, (codedBands + 1i32 - start) as opus_uint32))
                as c_int
        }
    } else {
        *intensity = 0i32
    }
    if *intensity <= start {
        total += dual_stereo_rsv;
        dual_stereo_rsv = 0i32
    }
    if dual_stereo_rsv > 0i32 {
        if 0 != encode {
            ec_enc_bit_logp(ec, *dual_stereo, 1i32 as c_uint);
        } else {
            *dual_stereo = ec_dec_bit_logp(ec, 1i32 as c_uint)
        }
    } else {
        *dual_stereo = 0i32
    }
    left = total - psum;
    percoeff = celt_udiv(
        left as opus_uint32,
        (i32::from(*(*m).eBands.offset(codedBands as isize))
            - i32::from(*(*m).eBands.offset(start as isize))) as opus_uint32,
    ) as opus_int32;
    left -= (i32::from(*(*m).eBands.offset(codedBands as isize))
        - i32::from(*(*m).eBands.offset(start as isize)))
        * percoeff;
    j = start;
    while j < codedBands {
        *bits.offset(j as isize) += percoeff
            * (i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                - i32::from(*(*m).eBands.offset(j as isize)));
        j += 1
    }
    j = start;
    while j < codedBands {
        let tmp_1: c_int = if left
            < i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                - i32::from(*(*m).eBands.offset(j as isize))
        {
            left
        } else {
            i32::from(*(*m).eBands.offset((j + 1i32) as isize))
                - i32::from(*(*m).eBands.offset(j as isize))
        };
        *bits.offset(j as isize) += tmp_1;
        left -= tmp_1;
        j += 1
    }
    balance = 0i32;
    j = start;
    while j < codedBands {
        let N0: c_int;
        let N: c_int;
        let den: c_int;
        let mut offset: c_int;
        let NClogN: c_int;
        let mut excess: opus_int32;
        let bit: opus_int32;
        if *bits.offset(j as isize) < 0i32 {
            eprintln!("Fatal (internal) error in rate.rs\n");
            panic!()
        }
        N0 = i32::from(*(*m).eBands.offset((j + 1i32) as isize))
            - i32::from(*(*m).eBands.offset(j as isize));
        N = N0 << LM;
        bit = *bits.offset(j as isize) + balance;
        if N > 1i32 {
            excess = if bit - *cap.offset(j as isize) > 0i32 {
                bit - *cap.offset(j as isize)
            } else {
                0i32
            };
            *bits.offset(j as isize) = bit - excess;
            den = C * N
                + if C == 2i32 && N > 2i32 && 0 == *dual_stereo && j < *intensity {
                    1i32
                } else {
                    0i32
                };
            NClogN = den * (i32::from(*(*m).logN.offset(j as isize)) + logM);
            offset = (NClogN >> 1i32) - den * 21i32;
            if N == 2i32 {
                offset += den << 3i32 >> 2i32
            }
            if *bits.offset(j as isize) + offset < (den * 2i32) << 3i32 {
                offset += NClogN >> 2i32
            } else if *bits.offset(j as isize) + offset < (den * 3i32) << 3i32 {
                offset += NClogN >> 3i32
            }
            *ebits.offset(j as isize) =
                if 0i32 > *bits.offset(j as isize) + offset + (den << (3i32 - 1i32)) {
                    0i32
                } else {
                    *bits.offset(j as isize) + offset + (den << (3i32 - 1i32))
                };
            *ebits.offset(j as isize) =
                (celt_udiv(*ebits.offset(j as isize) as opus_uint32, den as opus_uint32) >> 3i32)
                    as c_int;
            if C * *ebits.offset(j as isize) > *bits.offset(j as isize) >> 3i32 {
                *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3i32
            }
            *ebits.offset(j as isize) = if *ebits.offset(j as isize) < 8i32 {
                *ebits.offset(j as isize)
            } else {
                8i32
            };
            *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) * (den << 3i32)
                >= *bits.offset(j as isize) + offset)
                as c_int;
            *bits.offset(j as isize) -= (C * *ebits.offset(j as isize)) << 3i32
        } else {
            excess = if 0i32 > bit - (C << 3i32) {
                0i32
            } else {
                bit - (C << 3i32)
            };
            *bits.offset(j as isize) = bit - excess;
            *ebits.offset(j as isize) = 0i32;
            *fine_priority.offset(j as isize) = 1i32
        }
        if excess > 0i32 {
            let extra_fine: c_int;
            let extra_bits: c_int;
            extra_fine = if excess >> (stereo + 3i32) < 8i32 - *ebits.offset(j as isize) {
                excess >> (stereo + 3i32)
            } else {
                8i32 - *ebits.offset(j as isize)
            };
            *ebits.offset(j as isize) += extra_fine;
            extra_bits = (extra_fine * C) << 3i32;
            *fine_priority.offset(j as isize) = (extra_bits >= excess - balance) as c_int;
            excess -= extra_bits
        }
        balance = excess;
        if *bits.offset(j as isize) < 0i32 {
            eprintln!("Fatal (internal) error in rate.rs\n");
            panic!()
        }
        if *ebits.offset(j as isize) < 0i32 {
            eprintln!("Fatal (internal) error in rate.rs\n");
            panic!()
        }
        j += 1
    }
    *_balance = balance;
    while j < end {
        *ebits.offset(j as isize) = *bits.offset(j as isize) >> stereo >> 3i32;
        if (C * *ebits.offset(j as isize)) << 3i32 != *bits.offset(j as isize) {
            eprintln!("Fatal (internal) error in rate.rs\n");
            panic!()
        }
        *bits.offset(j as isize) = 0i32;
        *fine_priority.offset(j as isize) = (*ebits.offset(j as isize) < 1i32) as c_int;
        j += 1
    }
    codedBands
}

static mut LOG2_FRAC_TABLE: [c_uchar; 24] = [
    0, 8, 13, 16, 19, 21, 23, 24, 26, 27, 28, 29, 30, 31, 32, 32, 33, 34, 34, 35, 36, 36, 37, 37,
];
