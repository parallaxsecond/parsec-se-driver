#!/usr/bin/env bash

# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

# Continuous Integration test script, executed by GitHub Actions on x86 and
# Travis CI on Arm64.

set -euf -o pipefail

################
# Build client #
################
RUST_BACKTRACE=1 cargo build

#################
# Static checks #
#################
# On native target clippy or fmt might not be available.
if cargo fmt -h; then
	cargo fmt --all -- --check
fi
if cargo clippy -h; then
	cargo clippy --all-targets -- -D clippy::all -D clippy::cargo
fi

#############
# Run tests #
#############
RUST_BACKTRACE=1 cargo test

###########
# C Tests #
###########
# Build the driver
cargo build
# Compile Mbed Crypto (use the one in OUT_DIR)
MBED_TLS_PATH=`find target -name "mbedtls-mbedtls-*"`
pushd $MBED_TLS_PATH
./scripts/config.py crypto
./scripts/config.py set MBEDTLS_PSA_CRYPTO_SE_C
make
popd
# Compile and run the C application
make -C c-tests run MBED_TLS_PATH=$MBED_TLS_PATH
