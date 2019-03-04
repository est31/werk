// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

extern crate cc;

use cc::Build;

macro_rules! program {
	($($($pathseg:ident)/ *.c),*; $libname:expr) => {{
		let mut cfg = Build::new();
		cfg
			.include("../libopus")
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
		cfg $(.file(concat!("../libopus/",
			$("/", stringify!($pathseg)),*,
			".c"
		)))*;
		cfg.compile($libname);
	}}
}

fn celt_tests() {
	program! {
		celt/tests/test_unit_cwrs32.c;
		"libcelt_cwrs.a"
	}
	program! {
		celt/tests/test_unit_dft.c;
		"libcelt_dft.a"
	}
	program! {
		celt/tests/test_unit_entropy.c;
		"libcelt_entropy.a"
	}
	program! {
		celt/tests/test_unit_laplace.c;
		"libcelt_laplace.a"
	}
	program! {
		celt/tests/test_unit_mathops.c;
		"libcelt_mathops.a"
	}
	program! {
		celt/tests/test_unit_mdct.c;
		"libcelt_mdct.a"
	}
	program! {
		celt/tests/test_unit_rotation.c;
		"libcelt_rotation.a"
	}
	program! {
		celt/tests/test_unit_types.c;
		"libcelt_types.a"
	}
}

fn silk_tests() {
	program! {
		silk/tests/test_unit_LPC_inv_pred_gain.c;
		"libsilk_inv_pred_gain.a"
	}
}

fn opus_tests() {
	program! {
		tests/test_opus_decode.c;
		"libopus_decode.a"
	}

	program! {
		tests/test_opus_encode.c,
		tests/opus_encode_regressions.c;
		"libopus_encode.a"
	}

	program! {
		tests/test_opus_api.c;
		"libopus_api.a"
	}

	program! {
		tests/test_opus_padding.c;
		"libopus_padding.a"
	}
}

fn tools() {
	program! {
		src/opus_compare.c;
		"libopus_compare.a"
	}

	program! {
		src/opus_demo.c;
		"libopus_demo.a"
	}
}

fn main() {
	celt_tests();
	silk_tests();
	opus_tests();
	tools();
}
