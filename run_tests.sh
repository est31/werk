#!/bin/sh
# Werk - a pure Rust opus library
#
# Copyright (c) 2001-2011 the opus developers, and
# Copyright (c) 2017 est31 <MTest31@outlook.com>
# and contributors, All rights reserved.
# Licensed under the BSD 3 clause license.
# Please see the COPYING file attached to
# this source distribution for details.

cur_dir=`dirname "$0"`
cd "$cur_dir/werk_test"

cargo run --release --bin test_opus_decode
cargo run --release --bin test_opus_encode
cargo run --release --bin test_opus_api
cargo run --release --bin test_opus_padding
