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
	($name:ident; $($($pathseg:ident)/ *.c),*; $libname:expr) => {
		fn $name() {
			let mut cfg = Config::new();
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
			cfg $(.file(concat!("../libopus/",
				$("/", stringify!($pathseg)),*,
				".c"
			)))*;
			cfg.compile($libname);
		}
	}
}

test_program! { compile_test_opus_decode;
	tests/test_opus_decode.c;
	"libopus_decode.a"
}

test_program! { compile_test_opus_encode;
	tests/test_opus_encode.c,
	tests/opus_encode_regressions.c;
	"libopus_encode.a"
}

// TODO: add other tests


fn main() {
	compile_test_opus_decode();
	compile_test_opus_encode();
}
