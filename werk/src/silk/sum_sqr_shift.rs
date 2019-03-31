
pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type opus_int16 = int16_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;
fn silk_CLZ32(mut in32: opus_int32) -> opus_int32 {
    return if 0 != in32 {
               32 -
                  unsafe{ (::std::mem::size_of::<c_uint>() as c_ulong as
                        c_int * 8 -
                        (in32 as c_uint).leading_zeros() as ) }
           } else { 32 };
}
/* Compute number of bits to right shift the sum of squares of a vector    */
/* of int16s to make it fit in an int32                                    */
#[no_mangle]
pub fn silk_sum_sqr_shift(mut energy: *mut opus_int32,
                                            mut shift: *mut c_int,
                                            mut x: *const opus_int16,
                                            mut len: c_int) {
    let mut i: c_int = 0;
    let mut shft: c_int = 0;
    let mut nrg_tmp: opus_uint32 = 0;
    let mut nrg: opus_int32 = 0;
    unsafe {shft = 31 - silk_CLZ32(len);}
    nrg = len;
    i = 0;
    while i < len - 1 {
        nrg_tmp =
            (*x.offset(i as isize) as opus_int32 *
                 *x.offset(i as isize) as opus_int32) as opus_uint32;
        nrg_tmp =
            nrg_tmp.wrapping_add((*x.offset((i + 1) as isize) as opus_int32
                                      *
                                      *x.offset((i + 1) as isize) as
                                          opus_int32) as opus_uint32) as
                opus_int32 as opus_uint32;
        nrg =
            (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2
    }
    if i < len {
        nrg_tmp =
            (*x.offset(i as isize) as opus_int32 *
                 *x.offset(i as isize) as opus_int32) as opus_uint32;
        nrg =
            (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32
    }
    shft = silk_max_32(0, shft + 3 - silk_CLZ32(nrg));
    nrg = 0;
    i = 0;
    while i < len - 1 {
        nrg_tmp =
            (*x.offset(i as isize) as opus_int32 *
                 *x.offset(i as isize) as opus_int32) as opus_uint32;
        nrg_tmp =
            nrg_tmp.wrapping_add((*x.offset((i + 1) as isize) as opus_int32
                                      *
                                      *x.offset((i + 1) as isize) as
                                          opus_int32) as opus_uint32) as
                opus_int32 as opus_uint32;
        nrg =
            (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32;
        i += 2
    }
    if i < len {
        nrg_tmp =
            (*x.offset(i as isize) as opus_int32 *
                 *x.offset(i as isize) as opus_int32) as opus_uint32;
        nrg =
            (nrg as c_uint).wrapping_add(nrg_tmp >> shft) as opus_int32
    }
    *shift = shft;
    *energy = nrg;
}
unsafe extern "C" fn silk_max_32(mut a: opus_int32, mut b: opus_int32)
 -> opus_int32 {
    return if a > b { a } else { b };
}