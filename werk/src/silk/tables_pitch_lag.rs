use std::os::raw::*;
pub type __uint8_t = c_uchar;
pub type uint8_t = __uint8_t;
pub type opus_uint8 = uint8_t;
/* 32 */
#[no_mangle]
pub static mut silk_pitch_lag_iCDF: [opus_uint8; 32] =
    [253, 250, 244,
     233, 212, 182,
     150, 131, 120,
     110, 98, 85,
     72, 60, 49,
     40, 32, 25,
     19, 15, 13,
     11, 9, 8,
     7, 6, 5,
     4, 3, 2,
     1, 0];
/*  21 */
#[no_mangle]
pub static mut silk_pitch_delta_iCDF: [opus_uint8; 21] =
    [210, 208, 206,
     203, 199, 193,
     183, 168, 142,
     104, 74, 52,
     37, 27, 20,
     14, 10, 6,
     4, 2, 0];
/*  34 */
#[no_mangle]
pub static mut silk_pitch_contour_iCDF: [opus_uint8; 34] =
    [223, 201, 183,
     167, 152, 138,
     124, 111, 98,
     88, 79, 70,
     62, 56, 50,
     44, 39, 35,
     31, 27, 24,
     21, 18, 16,
     14, 12, 10,
     8, 6, 4,
     3, 2, 1,
     0];
/*  11 */
#[no_mangle]
pub static mut silk_pitch_contour_NB_iCDF: [opus_uint8; 11] =
    [188, 176, 155,
     138, 119, 97,
     67, 43, 26,
     10, 0];
/*  12 */
#[no_mangle]
pub static mut silk_pitch_contour_10_ms_iCDF: [opus_uint8; 12] =
    [165, 119, 80,
     61, 47, 35,
     27, 20, 14,
     9, 4, 0];
/*   3 */
#[no_mangle]
pub static mut silk_pitch_contour_10_ms_NB_iCDF: [opus_uint8; 3] =
    [113, 63, 0];