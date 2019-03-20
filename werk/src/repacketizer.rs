#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::*;

extern "C" {
    /* * Gets the number of samples per frame from an Opus packet.
     * @param [in] data <tt>char*</tt>: Opus packet.
     *                                  This must contain at least one byte of
     *                                  data.
     * @param [in] Fs <tt>opus_int32</tt>: Sampling rate in Hz.
     *                                     This must be a multiple of 400, or
     *                                     inaccurate results will be returned.
     * @returns Number of samples per frame.
     */
    #[no_mangle]
    fn opus_packet_get_samples_per_frame(data: *const c_uchar, Fs: opus_int32) -> c_int;
    /* * Gets the number of frames in an Opus packet.
     * @param [in] packet <tt>char*</tt>: Opus packet
     * @param [in] len <tt>opus_int32</tt>: Length of packet
     * @returns Number of frames
     * @retval OPUS_BAD_ARG Insufficient data was passed to the function
     * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
     */
    #[no_mangle]
    fn opus_packet_get_nb_frames(packet: *const c_uchar, len: opus_int32) -> c_int;
    #[no_mangle]
    fn malloc(_: c_ulong) -> *mut c_void;
    #[no_mangle]
    fn free(__ptr: *mut c_void);
    #[no_mangle]
    fn opus_packet_parse_impl(
        data: *const c_uchar,
        len: opus_int32,
        self_delimited: c_int,
        out_toc: *mut c_uchar,
        frames: *mut *const c_uchar,
        size: *mut opus_int16,
        payload_offset: *mut c_int,
        packet_offset: *mut opus_int32,
    ) -> c_int;
    #[no_mangle]
    fn memmove(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
    #[no_mangle]
    fn encode_size(size: c_int, data: *mut c_uchar) -> c_int;
}
pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type opus_int16 = int16_t;
pub type opus_int32 = int32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpusRepacketizer {
    pub toc: c_uchar,
    pub nb_frames: c_int,
    pub frames: [*const c_uchar; 48],
    pub len: [opus_int16; 48],
    pub framesize: c_int,
}
pub type size_t = c_ulong;
/* * Gets the size of an <code>OpusRepacketizer</code> structure.
 * @returns The size in bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_get_size() -> c_int {
    ::std::mem::size_of::<OpusRepacketizer>() as c_ulong as c_int
}

/* * (Re)initializes a previously allocated repacketizer state.
 * The state must be at least the size returned by opus_repacketizer_get_size().
 * This can be used for applications which use their own allocator instead of
 * malloc().
 * It must also be called to reset the queue of packets waiting to be
 * repacketized, which is necessary if the maximum packet duration of 120 ms
 * is reached or if you wish to submit packets with a different Opus
 * configuration (coding mode, audio bandwidth, frame size, or channel count).
 * Failure to do so will prevent a new packet from being added with
 * opus_repacketizer_cat().
 * @see opus_repacketizer_create
 * @see opus_repacketizer_get_size
 * @see opus_repacketizer_cat
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state to
 *                                       (re)initialize.
 * @returns A pointer to the same repacketizer state that was passed in.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_init(
    rp: *mut OpusRepacketizer,
) -> *mut OpusRepacketizer {
    (*rp).nb_frames = 0i32;
    rp
}

/* * Allocates memory and initializes the new repacketizer with
 * opus_repacketizer_init().
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_create() -> *mut OpusRepacketizer {
    let rp: *mut OpusRepacketizer;
    rp = opus_alloc(opus_repacketizer_get_size() as size_t) as *mut OpusRepacketizer;
    if rp.is_null() {
        return std::ptr::null_mut();
    }
    opus_repacketizer_init(rp)
}

/* * Opus wrapper for malloc(). To do your own dynamic allocation, all you need to do is replace this function and opus_free */
unsafe extern "C" fn opus_alloc(size: size_t) -> *mut c_void {
    malloc(size)
}

/* * Frees an <code>OpusRepacketizer</code> allocated by
 * opus_repacketizer_create().
 * @param[in] rp <tt>OpusRepacketizer*</tt>: State to be freed.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_destroy(rp: *mut OpusRepacketizer) {
    opus_free(rp as *mut c_void);
}
/* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
unsafe extern "C" fn opus_free(ptr: *mut c_void) {
    free(ptr);
}

/* * Add a packet to the current repacketizer state.
 * This packet must match the configuration of any packets already submitted
 * for repacketization since the last call to opus_repacketizer_init().
 * This means that it must have the same coding mode, audio bandwidth, frame
 * size, and channel count.
 * This can be checked in advance by examining the top 6 bits of the first
 * byte of the packet, and ensuring they match the top 6 bits of the first
 * byte of any previously submitted packet.
 * The total duration of audio in the repacketizer state also must not exceed
 * 120 ms, the maximum duration of a single packet, after adding this packet.
 *
 * The contents of the current repacketizer state can be extracted into new
 * packets using opus_repacketizer_out() or opus_repacketizer_out_range().
 *
 * In order to add a packet with a different configuration or to add more
 * audio beyond 120 ms, you must clear the repacketizer state by calling
 * opus_repacketizer_init().
 * If a packet is too large to add to the current repacketizer state, no part
 * of it is added, even if it contains multiple frames, some of which might
 * fit.
 * If you wish to be able to add parts of such packets, you should first use
 * another repacketizer to split the packet into pieces and add them
 * individually.
 * @see opus_repacketizer_out_range
 * @see opus_repacketizer_out
 * @see opus_repacketizer_init
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state to which to
 *                                       add the packet.
 * @param[in] data <tt>const unsigned char*</tt>: The packet data.
 *                                                The application must ensure
 *                                                this pointer remains valid
 *                                                until the next call to
 *                                                opus_repacketizer_init() or
 *                                                opus_repacketizer_destroy().
 * @param len <tt>opus_int32</tt>: The number of bytes in the packet data.
 * @returns An error code indicating whether or not the operation succeeded.
 * @retval #OPUS_OK The packet's contents have been added to the repacketizer
 *                  state.
 * @retval #OPUS_INVALID_PACKET The packet did not have a valid TOC sequence,
 *                              the packet's TOC sequence was not compatible
 *                              with previously submitted packets (because
 *                              the coding mode, audio bandwidth, frame size,
 *                              or channel count did not match), or adding
 *                              this packet would increase the total amount of
 *                              audio stored in the repacketizer state to more
 *                              than 120 ms.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_cat(
    rp: *mut OpusRepacketizer,
    data: *const c_uchar,
    len: opus_int32,
) -> c_int {
    opus_repacketizer_cat_impl(rp, data, len, 0i32)
}

unsafe extern "C" fn opus_repacketizer_cat_impl(
    rp: *mut OpusRepacketizer,
    data: *const c_uchar,
    len: opus_int32,
    self_delimited: c_int,
) -> c_int {
    let mut tmp_toc: c_uchar = 0;
    let curr_nb_frames: c_int;
    let ret: c_int;
    if len < 1i32 {
        return -4i32;
    }
    if (*rp).nb_frames == 0i32 {
        (*rp).toc = *data.offset(0isize);
        (*rp).framesize = opus_packet_get_samples_per_frame(data, 8000i32)
    } else if i32::from((*rp).toc) & 0xfci32 != i32::from(*data.offset(0isize)) & 0xfci32 {
        return -4i32;
    }
    curr_nb_frames = opus_packet_get_nb_frames(data, len);
    if curr_nb_frames < 1i32 {
        return -4i32;
    }
    if (curr_nb_frames + (*rp).nb_frames) * (*rp).framesize > 960i32 {
        return -4i32;
    }
    ret = opus_packet_parse_impl(
        data,
        len,
        self_delimited,
        &mut tmp_toc,
        &mut (*rp).frames[(*rp).nb_frames as usize],
        &mut (*rp).len[(*rp).nb_frames as usize],
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if ret < 1i32 {
        return ret;
    }
    (*rp).nb_frames += curr_nb_frames;
    0i32
}

/* * Construct a new packet from data previously submitted to the repacketizer
 * state via opus_repacketizer_cat().
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state from which to
 *                                       construct the new packet.
 * @param begin <tt>int</tt>: The index of the first frame in the current
 *                            repacketizer state to include in the output.
 * @param end <tt>int</tt>: One past the index of the last frame in the
 *                          current repacketizer state to include in the
 *                          output.
 * @param[out] data <tt>const unsigned char*</tt>: The buffer in which to
 *                                                 store the output packet.
 * @param maxlen <tt>opus_int32</tt>: The maximum number of bytes to store in
 *                                    the output buffer. In order to guarantee
 *                                    success, this should be at least
 *                                    <code>1276</code> for a single frame,
 *                                    or for multiple frames,
 *                                    <code>1277*(end-begin)</code>.
 *                                    However, <code>1*(end-begin)</code> plus
 *                                    the size of all packet data submitted to
 *                                    the repacketizer since the last call to
 *                                    opus_repacketizer_init() or
 *                                    opus_repacketizer_create() is also
 *                                    sufficient, and possibly much smaller.
 * @returns The total size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG <code>[begin,end)</code> was an invalid range of
 *                       frames (begin < 0, begin >= end, or end >
 *                       opus_repacketizer_get_nb_frames()).
 * @retval #OPUS_BUFFER_TOO_SMALL \a maxlen was insufficient to contain the
 *                                complete output packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_out_range(
    rp: *mut OpusRepacketizer,
    begin: c_int,
    end: c_int,
    data: *mut c_uchar,
    maxlen: opus_int32,
) -> opus_int32 {
    opus_repacketizer_out_range_impl(rp, begin, end, data, maxlen, 0i32, 0i32)
}

#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_out_range_impl(
    rp: *mut OpusRepacketizer,
    begin: c_int,
    end: c_int,
    data: *mut c_uchar,
    maxlen: opus_int32,
    self_delimited: c_int,
    pad: c_int,
) -> opus_int32 {
    let mut i: c_int;
    let count: c_int;
    let mut tot_size: opus_int32;
    let len: *mut opus_int16;
    let frames: *mut *const c_uchar;
    let mut ptr: *mut c_uchar;
    if begin < 0i32 || begin >= end || end > (*rp).nb_frames {
        return -1i32;
    }
    count = end - begin;
    len = (*rp).len.as_mut_ptr().offset(begin as isize);
    frames = (*rp).frames.as_mut_ptr().offset(begin as isize);
    if 0 != self_delimited {
        tot_size = 1i32 + (i32::from(*len.offset((count - 1i32) as isize)) >= 252i32) as c_int
    } else {
        tot_size = 0i32
    }
    ptr = data;
    if count == 1i32 {
        tot_size += i32::from(*len.offset(0isize)) + 1i32;
        if tot_size > maxlen {
            return -2i32;
        }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = (i32::from((*rp).toc) & 0xfci32) as c_uchar
    } else if count == 2i32 {
        if i32::from(*len.offset(1isize)) == i32::from(*len.offset(0isize)) {
            tot_size += 2i32 * i32::from(*len.offset(0isize)) + 1i32;
            if tot_size > maxlen {
                return -2i32;
            }
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            *fresh1 = (i32::from((*rp).toc) & 0xfci32 | 0x1i32) as c_uchar
        } else {
            tot_size += i32::from(*len.offset(0isize))
                + i32::from(*len.offset(1isize))
                + 2i32
                + (i32::from(*len.offset(0isize)) >= 252i32) as c_int;
            if tot_size > maxlen {
                return -2i32;
            }
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = (i32::from((*rp).toc) & 0xfci32 | 0x2i32) as c_uchar;
            ptr = ptr.offset(encode_size(i32::from(*len.offset(0isize)), ptr) as isize)
        }
    }
    if count > 2i32 || 0 != pad && tot_size < maxlen {
        let mut vbr: c_int;
        let pad_amount: c_int;
        ptr = data;
        if 0 != self_delimited {
            tot_size = 1i32 + (i32::from(*len.offset((count - 1i32) as isize)) >= 252i32) as c_int
        } else {
            tot_size = 0i32
        }
        vbr = 0i32;
        i = 1i32;
        while i < count {
            if i32::from(*len.offset(i as isize)) != i32::from(*len.offset(0isize)) {
                vbr = 1i32;
                break;
            } else {
                i += 1
            }
        }
        if 0 != vbr {
            tot_size += 2i32;
            i = 0i32;
            while i < count - 1i32 {
                tot_size += 1i32
                    + (i32::from(*len.offset(i as isize)) >= 252i32) as c_int
                    + i32::from(*len.offset(i as isize));
                i += 1
            }
            tot_size += i32::from(*len.offset((count - 1i32) as isize));
            if tot_size > maxlen {
                return -2i32;
            }
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = (i32::from((*rp).toc) & 0xfci32 | 0x3i32) as c_uchar;
            let fresh4 = ptr;
            ptr = ptr.offset(1);
            *fresh4 = (count | 0x80i32) as c_uchar
        } else {
            tot_size += count * i32::from(*len.offset(0isize)) + 2i32;
            if tot_size > maxlen {
                return -2i32;
            }
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            *fresh5 = (i32::from((*rp).toc) & 0xfci32 | 0x3i32) as c_uchar;
            let fresh6 = ptr;
            ptr = ptr.offset(1);
            *fresh6 = count as c_uchar
        }
        pad_amount = if 0 != pad { maxlen - tot_size } else { 0i32 };
        if pad_amount != 0i32 {
            let nb_255s: c_int;
            let fresh7 = &mut (*data.offset(1isize));
            *fresh7 = (i32::from(*fresh7) | 0x40i32) as c_uchar;
            nb_255s = (pad_amount - 1i32) / 255i32;
            i = 0i32;
            while i < nb_255s {
                let fresh8 = ptr;
                ptr = ptr.offset(1);
                *fresh8 = 255i32 as c_uchar;
                i += 1
            }
            let fresh9 = ptr;
            ptr = ptr.offset(1);
            *fresh9 = (pad_amount - 255i32 * nb_255s - 1i32) as c_uchar;
            tot_size += pad_amount
        }
        if 0 != vbr {
            i = 0i32;
            while i < count - 1i32 {
                ptr = ptr.offset(encode_size(i32::from(*len.offset(i as isize)), ptr) as isize);
                i += 1
            }
        }
    }
    if 0 != self_delimited {
        let sdlen: c_int = encode_size(i32::from(*len.offset((count - 1i32) as isize)), ptr);
        ptr = ptr.offset(sdlen as isize)
    }
    i = 0i32;
    while i < count {
        memmove(
            ptr as *mut c_void,
            *frames.offset(i as isize) as *const c_void,
            (*len.offset(i as isize) as c_ulong)
                .wrapping_mul(::std::mem::size_of::<c_uchar>() as c_ulong),
        );
        ptr = ptr.offset(i32::from(*len.offset(i as isize)) as isize);
        i += 1
    }
    if 0 != pad {
        while ptr < data.offset(maxlen as isize) {
            let fresh10 = ptr;
            ptr = ptr.offset(1);
            *fresh10 = 0i32 as c_uchar
        }
    }
    tot_size
}

/* * Return the total number of frames contained in packet data submitted to
 * the repacketizer state so far via opus_repacketizer_cat() since the last
 * call to opus_repacketizer_init() or opus_repacketizer_create().
 * This defines the valid range of packets that can be extracted with
 * opus_repacketizer_out_range() or opus_repacketizer_out().
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state containing the
 *                                       frames.
 * @returns The total number of frames contained in the packet data submitted
 *          to the repacketizer state.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_get_nb_frames(rp: *mut OpusRepacketizer) -> c_int {
    (*rp).nb_frames
}

/* * Construct a new packet from data previously submitted to the repacketizer
 * state via opus_repacketizer_cat().
 * This is a convenience routine that returns all the data submitted so far
 * in a single packet.
 * It is equivalent to calling
 * @code
 * opus_repacketizer_out_range(rp, 0, opus_repacketizer_get_nb_frames(rp),
 *                             data, maxlen)
 * @endcode
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state from which to
 *                                       construct the new packet.
 * @param[out] data <tt>const unsigned char*</tt>: The buffer in which to
 *                                                 store the output packet.
 * @param maxlen <tt>opus_int32</tt>: The maximum number of bytes to store in
 *                                    the output buffer. In order to guarantee
 *                                    success, this should be at least
 *                                    <code>1277*opus_repacketizer_get_nb_frames(rp)</code>.
 *                                    However,
 *                                    <code>1*opus_repacketizer_get_nb_frames(rp)</code>
 *                                    plus the size of all packet data
 *                                    submitted to the repacketizer since the
 *                                    last call to opus_repacketizer_init() or
 *                                    opus_repacketizer_create() is also
 *                                    sufficient, and possibly much smaller.
 * @returns The total size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BUFFER_TOO_SMALL \a maxlen was insufficient to contain the
 *                                complete output packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_repacketizer_out(
    rp: *mut OpusRepacketizer,
    data: *mut c_uchar,
    maxlen: opus_int32,
) -> opus_int32 {
    opus_repacketizer_out_range_impl(rp, 0i32, (*rp).nb_frames, data, maxlen, 0i32, 0i32)
}

/* * Pads a given Opus packet to a larger size (possibly changing the TOC sequence).
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to pad.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param new_len <tt>opus_int32</tt>: The desired size of the packet after padding.
 *                                 This must be at least as large as len.
 * @returns an error code
 * @retval #OPUS_OK \a on success.
 * @retval #OPUS_BAD_ARG \a len was less than 1 or new_len was less than len.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_packet_pad(
    data: *mut c_uchar,
    len: opus_int32,
    new_len: opus_int32,
) -> c_int {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [std::ptr::null(); 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: opus_int32;
    if len < 1i32 {
        return -1i32;
    }
    if len == new_len {
        return 0i32;
    } else if len > new_len {
        return -1i32;
    }
    opus_repacketizer_init(&mut rp);
    memmove(
        data.offset(new_len as isize).offset(-(len as isize)) as *mut c_void,
        data as *const c_void,
        (len as c_ulong).wrapping_mul(::std::mem::size_of::<c_uchar>() as c_ulong),
    );
    ret = opus_repacketizer_cat(
        &mut rp,
        data.offset(new_len as isize).offset(-(len as isize)),
        len,
    );
    if ret != 0i32 {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(&mut rp, 0i32, rp.nb_frames, data, new_len, 0i32, 1i32);
    if ret > 0i32 {
        0i32
    } else {
        ret
    }
}

/* * Remove all padding from a given Opus packet and rewrite the TOC sequence to
 * minimize space usage.
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to strip.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @returns The new size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG \a len was less than 1.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_packet_unpad(data: *mut c_uchar, len: opus_int32) -> opus_int32 {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [std::ptr::null(); 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: opus_int32;
    if len < 1i32 {
        return -1i32;
    }
    opus_repacketizer_init(&mut rp);
    ret = opus_repacketizer_cat(&mut rp, data, len);
    if ret < 0i32 {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(&mut rp, 0i32, rp.nb_frames, data, len, 0i32, 0i32);
    if !(ret > 0i32 && ret <= len) {
        eprintln!("Fatal (internal) error in repacketizer.rs\n");
        panic!()
    }
    ret
} /* Copyright (C) 2007 Jean-Marc Valin

     File: os_support.h
     This is the (tiny) OS abstraction layer. Aside from math.h, this is the
     only place where system headers are allowed.

     Redistribution and use in source and binary forms, with or without
     modification, are permitted provided that the following conditions are
     met:

     1. Redistributions of source code must retain the above copyright notice,
     this list of conditions and the following disclaimer.

     2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.

     THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
     IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
     OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
     DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
     INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
     (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
     SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
     STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
     POSSIBILITY OF SUCH DAMAGE.
  */

/* * Pads a given Opus multi-stream packet to a larger size (possibly changing the TOC sequence).
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to pad.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param new_len <tt>opus_int32</tt>: The desired size of the packet after padding.
 *                                 This must be at least 1.
 * @param nb_streams <tt>opus_int32</tt>: The number of streams (not channels) in the packet.
 *                                 This must be at least as large as len.
 * @returns an error code
 * @retval #OPUS_OK \a on success.
 * @retval #OPUS_BAD_ARG \a len was less than 1.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_multistream_packet_pad(
    mut data: *mut c_uchar,
    mut len: opus_int32,
    new_len: opus_int32,
    nb_streams: c_int,
) -> c_int {
    let mut s: c_int;
    let mut count: c_int;
    let mut toc: c_uchar = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    let mut packet_offset: opus_int32 = 0;
    let amount: opus_int32;
    if len < 1i32 {
        return -1i32;
    }
    if len == new_len {
        return 0i32;
    } else if len > new_len {
        return -1i32;
    }
    amount = new_len - len;
    s = 0i32;
    while s < nb_streams - 1i32 {
        if len <= 0i32 {
            return -4i32;
        }
        count = opus_packet_parse_impl(
            data,
            len,
            1i32,
            &mut toc,
            std::ptr::null_mut(),
            size.as_mut_ptr(),
            std::ptr::null_mut(),
            &mut packet_offset,
        );
        if count < 0i32 {
            return count;
        }
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1
    }
    opus_packet_pad(data, len, len + amount)
}

/* * Remove all padding from a given Opus multi-stream packet and rewrite the TOC sequence to
 * minimize space usage.
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to strip.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param nb_streams <tt>opus_int32</tt>: The number of streams (not channels) in the packet.
 *                                 This must be at least 1.
 * @returns The new size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG \a len was less than 1 or new_len was less than len.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]
pub unsafe extern "C" fn opus_multistream_packet_unpad(
    mut data: *mut c_uchar,
    mut len: opus_int32,
    nb_streams: c_int,
) -> opus_int32 {
    let mut s: c_int;
    let mut toc: c_uchar = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    let mut packet_offset: opus_int32 = 0;
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [std::ptr::null(); 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut dst: *mut c_uchar;
    let mut dst_len: opus_int32;
    if len < 1i32 {
        return -1i32;
    }
    dst = data;
    dst_len = 0i32;
    s = 0i32;
    while s < nb_streams {
        let mut ret: opus_int32;
        let self_delimited: c_int = (s != nb_streams - 1i32) as c_int;
        if len <= 0i32 {
            return -4i32;
        }
        opus_repacketizer_init(&mut rp);
        ret = opus_packet_parse_impl(
            data,
            len,
            self_delimited,
            &mut toc,
            std::ptr::null_mut(),
            size.as_mut_ptr(),
            std::ptr::null_mut(),
            &mut packet_offset,
        );
        if ret < 0i32 {
            return ret;
        }
        ret = opus_repacketizer_cat_impl(&mut rp, data, packet_offset, self_delimited);
        if ret < 0i32 {
            return ret;
        }
        ret = opus_repacketizer_out_range_impl(
            &mut rp,
            0i32,
            rp.nb_frames,
            dst,
            len,
            self_delimited,
            0i32,
        );
        if ret < 0i32 {
            return ret;
        } else {
            dst_len += ret
        }
        dst = dst.offset(ret as isize);
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1
    }
    dst_len
}
