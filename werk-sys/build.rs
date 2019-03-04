// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cc::Build;

macro_rules! files_fn {
	($name:ident; $($($pathseg:ident)/ *.c),*,) => {
		files_fn!($name; $($($pathseg)/ *.c),*);
	};
	($name:ident; $($($pathseg:ident)/ *.c),*) => {
		fn $name(cfg :&mut Build) {
			cfg $(.file(concat!("../libopus/",
				$("/", stringify!($pathseg)),*,
				".c"
			)))*;
		}
	};
}

files_fn! { opus_files;
	src/opus.c,
	src/opus_decoder.c,
	src/opus_encoder.c,
	src/opus_multistream.c,
	src/opus_multistream_encoder.c,
	src/opus_multistream_decoder.c,
	src/repacketizer.c
}

files_fn! { opus_files_float;
	src/analysis.c,
	src/mlp.c,
	src/mlp_data.c
}

files_fn! { celt_files;
	celt/bands.c,
	celt/celt.c,
	celt/celt_encoder.c, // encoder -- not a priority ATM
	celt/celt_decoder.c,
	celt/cwrs.c,
	// celt/entcode.c, -- SMALL_DIV_TABLE still missing
	// celt/entdec.c,
	celt/entenc.c, // encoder -- not a priority ATM
	celt/kiss_fft.c, // before porting check whether crates on crates.io can do this
	// celt/laplace.c,
	// celt/mathops.c, -- fixed point stuff still missing
	celt/mdct.c, // before porting check whether crates on crates.io can do this
	celt/modes.c,
	// celt/pitch.c,
	// celt/celt_lpc.c,
	celt/quant_bands.c,
	celt/rate.c,
	// celt/vq.c
}

files_fn! { silk_files;
	silk/CNG.c,
	silk/code_signs.c,
	silk/init_decoder.c,
	silk/decode_core.c,
	silk/decode_frame.c,
	silk/decode_parameters.c,
	silk/decode_indices.c,
	silk/decode_pulses.c,
	silk/decoder_set_fs.c,
	silk/dec_API.c,
	silk/enc_API.c,
	silk/encode_indices.c,
	silk/encode_pulses.c,
	silk/gain_quant.c,
	silk/interpolate.c,
	silk/LP_variable_cutoff.c,
	silk/NLSF_decode.c,
	silk/NSQ.c,
	silk/NSQ_del_dec.c,
	silk/PLC.c,
	silk/shell_coder.c,
	silk/tables_gain.c,
	silk/tables_LTP.c,
	silk/tables_NLSF_CB_NB_MB.c,
	silk/tables_NLSF_CB_WB.c,
	silk/tables_other.c,
	silk/tables_pitch_lag.c,
	silk/tables_pulses_per_block.c,
	silk/VAD.c,
	silk/control_audio_bandwidth.c,
	silk/quant_LTP_gains.c,
	silk/VQ_WMat_EC.c,
	silk/HP_variable_cutoff.c,
	silk/NLSF_encode.c,
	silk/NLSF_VQ.c,
	silk/NLSF_unpack.c,
	silk/NLSF_del_dec_quant.c,
	silk/process_NLSFs.c,
	silk/stereo_LR_to_MS.c,
	silk/stereo_MS_to_LR.c,
	silk/check_control_input.c,
	silk/control_SNR.c,
	silk/init_encoder.c,
	silk/control_codec.c,
	silk/A2NLSF.c,
	silk/ana_filt_bank_1.c,
	silk/biquad_alt.c,
	silk/bwexpander_32.c,
	silk/bwexpander.c,
	silk/debug.c,
	silk/decode_pitch.c,
	silk/inner_prod_aligned.c,
	silk/lin2log.c,
	silk/log2lin.c,
	silk/LPC_analysis_filter.c,
	silk/LPC_inv_pred_gain.c,
	silk/table_LSF_cos.c,
	silk/NLSF2A.c,
	silk/NLSF_stabilize.c,
	silk/NLSF_VQ_weights_laroia.c,
	silk/pitch_est_tables.c,
	silk/resampler.c,
	silk/resampler_down2_3.c,
	silk/resampler_down2.c,
	silk/resampler_private_AR2.c,
	silk/resampler_private_down_FIR.c,
	silk/resampler_private_IIR_FIR.c,
	silk/resampler_private_up2_HQ.c,
	silk/resampler_rom.c,
	silk/sigm_Q15.c,
	silk/sort.c,
	silk/sum_sqr_shift.c,
	silk/stereo_decode_pred.c,
	silk/stereo_encode_pred.c,
	silk/stereo_find_predictor.c,
	silk/stereo_quant_pred.c,
	silk/LPC_fit.c
}

files_fn! { silk_files_float;
	silk/float/apply_sine_window_FLP.c,
	silk/float/corrMatrix_FLP.c,
	silk/float/encode_frame_FLP.c,
	silk/float/find_LPC_FLP.c,
	silk/float/find_LTP_FLP.c,
	silk/float/find_pitch_lags_FLP.c,
	silk/float/find_pred_coefs_FLP.c,
	silk/float/LPC_analysis_filter_FLP.c,
	silk/float/LTP_analysis_filter_FLP.c,
	silk/float/LTP_scale_ctrl_FLP.c,
	silk/float/noise_shape_analysis_FLP.c,
	silk/float/process_gains_FLP.c,
	silk/float/regularize_correlations_FLP.c,
	silk/float/residual_energy_FLP.c,
	silk/float/warped_autocorrelation_FLP.c,
	silk/float/wrappers_FLP.c,
	silk/float/autocorrelation_FLP.c,
	silk/float/burg_modified_FLP.c,
	silk/float/bwexpander_FLP.c,
	silk/float/energy_FLP.c,
	silk/float/inner_product_FLP.c,
	silk/float/k2a_FLP.c,
	silk/float/LPC_inv_pred_gain_FLP.c,
	silk/float/pitch_analysis_core_FLP.c,
	silk/float/scale_copy_vector_FLP.c,
	silk/float/scale_vector_FLP.c,
	silk/float/schur_FLP.c,
	silk/float/sort_FLP.c
}

fn compile_opus() {
	let mut cfg = cc::Build::new();
	cfg
		.include("../libopus/include")
		.include("../libopus/celt")
		.include("../libopus/silk")
		.include("../libopus/silk/fixed")
		.include("../libopus/silk/float")
		// Note in configure.ac there are a bunch of such variables defined,
		// each with AC_DEFINE.
		// But these two are the required ones
		.define("OPUS_BUILD", None)
		.define("USE_ALLOCA", None);
	// Always optimize, this is no fun otherwise
	cfg.opt_level(3);
	opus_files(&mut cfg);
	opus_files_float(&mut cfg);
	celt_files(&mut cfg);
	silk_files(&mut cfg);
	silk_files_float(&mut cfg);
	cfg.compile("libopus.a");
}

fn generate_bindings() {
	let bindings = bindgen::Builder::default()
		.header("src/ffi_wrapper.h")
		.clang_arg("-I../libopus/include")
		.clang_arg("-I../libopus/celt")
		.clang_arg("-I../libopus/silk")
		.clang_arg("-I../libopus/silk/fixed")
		.clang_arg("-I../libopus/silk/float")
		.generate()
		.expect("Unable to generate API bindings");
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings.rs file");
}

fn main() {
	compile_opus();
	generate_bindings();
}
