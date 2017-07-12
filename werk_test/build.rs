// Werk - a pure Rust opus library
//
// Copyright (c) 2001-2011 the opus developers, and
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors, All rights reserved.
// Licensed under the BSD 3 clause license.
// Please see the COPYING file attached to
// this source distribution for details.

extern crate gcc;

use gcc::Config;

macro_rules! test_program {
	($($($pathseg:ident)/ *.c),*; $libname:expr) => {{
		let mut cfg = Config::new();
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
	test_program! {
		celt/tests/test_unit_cwrs32.c;
		"libcelt_cwrs.a"
	}
	test_program! {
		celt/tests/test_unit_dft.c;
		"libcelt_dft.a"
	}
	test_program! {
		celt/tests/test_unit_entropy.c;
		"libcelt_entropy.a"
	}
	test_program! {
		celt/tests/test_unit_laplace.c;
		"libcelt_laplace.a"
	}
	test_program! {
		celt/tests/test_unit_mathops.c;
		"libcelt_mathops.a"
	}
	test_program! {
		celt/tests/test_unit_mdct.c;
		"libcelt_mdct.a"
	}
	test_program! {
		celt/tests/test_unit_rotation.c;
		"libcelt_rotation.a"
	}
	test_program! {
		celt/tests/test_unit_types.c;
		"libcelt_types.a"
	}
}

fn silk_tests() {
	test_program! {
		silk/tests/test_unit_LPC_inv_pred_gain.c;
		"libsilk_inv_pred_gain.a"
	}
}

fn opus_tests() {
	test_program! {
		tests/test_opus_decode.c;
		"libopus_decode.a"
	}

	test_program! {
		tests/test_opus_encode.c,
		tests/opus_encode_regressions.c;
		"libopus_encode.a"
	}

	test_program! {
		tests/test_opus_api.c;
		"libopus_api.a"
	}

	test_program! {
		tests/test_opus_padding.c;
		"libopus_padding.a"
	}
}

fn main() {
	celt_tests();
	silk_tests();
	opus_tests();
}
