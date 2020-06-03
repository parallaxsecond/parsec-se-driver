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

# Install and start the TPM server and Parsec
cargo install parsec-service --features tpm-provider
tpm_server &
sleep 5
tpm2_startup -c -T mssim 2>/dev/null
tpm2_changeauth -c owner tpm_pass 2>/dev/null
parsec-service --config ci/config.toml
sleep 5

# Compile Mbed Crypto (use the one in OUT_DIR)
git clone https://github.com/ARMmbed/mbedtls.git
pushd mbedtls
./scripts/config.py crypto
./scripts/config.py set MBEDTLS_PSA_CRYPTO_SE_C
make
popd

# Build the driver
MBEDTLS_LIB_DIR=$(pwd)/mbedtls/build/library MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo build

# Compile and run the C application
make -C ci/c-tests run MBED_TLS_PATH=$(pwd)/mbedtls
