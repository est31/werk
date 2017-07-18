# Werk - a pure Rust opus library
#
# Copyright (c) 2001-2011 the opus developers, and
# Copyright (c) 2017 est31 <MTest31@outlook.com>
# and contributors, All rights reserved.
# Licensed under the BSD 3 clause license.
# Please see the COPYING file attached to
# this source distribution for details.

.SECONDEXPANSION:

tests=$(shell ls src/bin | sort | cut -d. -f1 | grep test)
build_tests=$(patsubst test_%,build_test_%,$(tests))

$(build_tests):
	echo "Building $(patsubst build_test_%,test_%,$@)"
	cargo build --release --bin \
	$(patsubst build_test_%,test_%,$@) >> data/logs/$@.log 2>&1

$(tests): data/logs #$$(patsubst test_%,build_test_%,$$@)
	echo "Running $@"
	cargo run --release --bin $@ >> data/logs/$@.log 2>&1

data:
	mkdir -p data

data/logs:
	mkdir -p data/logs

data/opus_testvectors.tar.gz:
	mkdir -p data
	cd data && \
	curl -LO https://opus-codec.org/testvectors/opus_testvectors.tar.gz

data/opus_testvectors: data/opus_testvectors.tar.gz
	cd data && \
	tar -zxf opus_testvectors.tar.gz

build_vectors_compare: data/logs
	cargo build --release --bin opus_compare >> data/logs/$@.log 2>&1

build_vectors_demo: data/logs
	cargo build --release --bin opus_demo >> data/logs/$@.log 2>&1

test_vectors: build_vectors_compare build_vectors_demo \
	data/opus_testvectors data/logs
	echo "Running $@"
	mkdir -p data/test_vectors_data
	cd data/test_vectors_data && \
	export RDIR=../../../target/release && \
	export OPUS_COMPARE=$${RDIR}/opus_compare && \
	export OPUS_DEMO=$${RDIR}/opus_demo && \
	../../../libopus/tests/run_vectors.sh . \
	../opus_testvectors 48000 >> ../logs/$@.log 2>&1

build_tests : $(build_tests) build_vectors
run_tests : $(tests) test_vectors

test: run_tests

clean_logs:
	rm -r data/logs
	rm -r data/test_vectors_data

clean:
	rm -r data

.PHONY: test run_tests build_tests $(tests) $(build_tests) \
	build_vectors_compare build_vectors_demo test_vectors \
	clean_logs clean

.SILENT:

# TODO: Improve error output (print when there is an error, don't just ask people to look in the logfile)
