#!/bin/sh
# Werk - a pure Rust opus library
#
# Copyright (c) 2001-2011 the opus developers, and
# Copyright (c) 2017 est31 <MTest31@outlook.com>
# and contributors, All rights reserved.
# Licensed under the BSD 3 clause license.
# Please see the COPYING file attached to
# this source distribution for details.

set -e

cur_dir=`dirname "$0"`
cur_dir=`realpath $cur_dir`
cd "$cur_dir/werk_test"

tests=`ls src/bin | sort | cut -d. -f1 | grep test`

for test in $tests; do
	echo "Running test $test"
	cargo run --release --bin $test
done

# Download the test vectors if required

mkdir -p data
cd data

if [ ! -e opus_testvectors.tar.gz ]; then
	curl -LO https://opus-codec.org/testvectors/opus_testvectors.tar.gz
fi
if [ ! -e opus_testvectors ]; then
	tar -zxf opus_testvectors.tar.gz
fi

# Now test the vectors

cargo build --release --bin opus_compare
cargo build --release --bin opus_demo

export OPUS_COMPARE="$cur_dir"/target/release/opus_compare
export OPUS_DEMO="$cur_dir"/target/release/opus_demo

../../libopus/tests/run_vectors.sh . opus_testvectors 48000
