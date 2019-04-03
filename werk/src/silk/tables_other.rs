/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
use std::os::raw::*;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __int32_t = c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type opus_uint8 = uint8_t;
pub type opus_int16 = int16_t;
pub type opus_int32 = int32_t;
/*   2 */
#[no_mangle]
pub static mut silk_lsb_iCDF: [opus_uint8; 2] =
    [120, 0];
/*   3 */
#[no_mangle]
pub static mut silk_uniform3_iCDF: [opus_uint8; 3] =
    [171, 85, 0];
/*   4 */
#[no_mangle]
pub static mut silk_uniform4_iCDF: [opus_uint8; 4] =
    [192, 128, 64,
     0];
/*   5 */
#[no_mangle]
pub static mut silk_uniform5_iCDF: [opus_uint8; 5] =
    [205, 154, 102,
     51, 0];
/*   6 */
#[no_mangle]
pub static mut silk_uniform6_iCDF: [opus_uint8; 6] =
    [213, 171, 128,
     85, 43, 0];
/*   8 */
#[no_mangle]
pub static mut silk_uniform8_iCDF: [opus_uint8; 8] =
    [224, 192, 160,
     128, 96, 64,
     32, 0];
/*   7 */
#[no_mangle]
pub static mut silk_NLSF_EXT_iCDF: [opus_uint8; 7] =
    [100, 40, 16,
     7, 3, 1,
     0];
/*   4 */
#[no_mangle]
pub static mut silk_LTPscale_iCDF: [opus_uint8; 3] =
    [128, 64, 0];
/*   6 */
#[no_mangle]
pub static mut silk_LTPScales_table_Q14: [opus_int16; 3] =
    [15565, 12288, 8192];
/*   4 */
#[no_mangle]
pub static mut silk_type_offset_VAD_iCDF: [opus_uint8; 4] =
    [232, 158, 10,
     0];
/*   2 */
#[no_mangle]
pub static mut silk_type_offset_no_VAD_iCDF: [opus_uint8; 2] =
    [230, 0];
/*  32 */
#[no_mangle]
pub static mut silk_stereo_pred_quant_Q13: [opus_int16; 16] =
    [-13732, -10050, -8266,
     -7526, -6500, -5000,
     -2950, -820, 820,
     2950, 5000, 6500,
     7526, 8266, 10050,
     13732];
/*  25 */
#[no_mangle]
pub static mut silk_stereo_pred_joint_iCDF: [opus_uint8; 25] =
    [249, 247, 246,
     245, 244, 234,
     210, 202, 201,
     200, 197, 174,
     82, 59, 56,
     55, 54, 46,
     22, 12, 11,
     10, 9, 7,
     0];
/*   2 */
#[no_mangle]
pub static mut silk_stereo_only_code_mid_iCDF: [opus_uint8; 2] =
    [64, 0];
/*  10 */
#[no_mangle]
pub static mut silk_LBRR_flags_iCDF_ptr: [*const opus_uint8; 2] =
    unsafe {
        [silk_LBRR_flags_2_iCDF.as_ptr(), silk_LBRR_flags_3_iCDF.as_ptr()]
    };
static mut silk_LBRR_flags_3_iCDF: [opus_uint8; 7] =
    [215, 195, 166,
     125, 110, 82,
     0];

/* Tables for stereo predictor coding */
/* Tables for LBRR flags */
static mut silk_LBRR_flags_2_iCDF: [opus_uint8; 3] =
    [203, 150, 0];
/*   5 */
#[no_mangle]
pub static mut silk_NLSF_interpolation_factor_iCDF: [opus_uint8; 5] =
    [243, 221, 192,
     181, 0];
/* Quantization offsets */
/*   8 */
#[no_mangle]
pub static mut silk_Quantization_Offsets_Q10: [[opus_int16; 2]; 2] =
    [[100, 240],
     [32, 100]];
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
/*  60 */
#[no_mangle]
pub static mut silk_Transition_LP_B_Q28: [[opus_int32; 3]; 5] =
    [[250767114, 501534038, 250767114],
     [209867381, 419732057, 209867381],
     [170987846, 341967853, 170987846],
     [131531482, 263046905, 131531482],
     [89306658, 178584282, 89306658]];
/*  60 */
#[no_mangle]
pub static mut silk_Transition_LP_A_Q28: [[opus_int32; 2]; 5] =
    [[506393414, 239854379], [411067935, 169683996],
     [306733530, 116694253], [185807084, 77959395],
     [35497197, 57401098]];