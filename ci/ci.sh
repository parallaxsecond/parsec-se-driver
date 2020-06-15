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

# Start the TPM server
tpm_server &
sleep 5
tpm2_startup -c -T mssim 2>/dev/null
tpm2_changeauth -c owner -T mssim tpm_pass 2>/dev/null
sleep 5

# Install and run Parsec
git clone https://github.com/parallaxsecond/parsec
pushd parsec
cargo build --features tpm-provider --release
./target/release/parsec -c ../ci/config.toml &
sleep 5
popd

# Compile Mbed Crypto
git clone https://github.com/ARMmbed/mbedtls.git
pushd mbedtls
git checkout mbedtls-2.22.0
./scripts/config.py crypto
./scripts/config.py set MBEDTLS_PSA_CRYPTO_SE_C
SHARED=1 make
popd

# Build the driver, clean before to force dynamic linking
cargo clean
MBEDTLS_LIB_DIR=$(pwd)/mbedtls/library MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo build --release

# Compile and run the C application
make -C ci/c-tests run MBED_TLS_PATH=$(pwd)/mbedtls
