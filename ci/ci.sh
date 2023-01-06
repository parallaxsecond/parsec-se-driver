#!/usr/bin/env bash

# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

# Continuous Integration test script, executed by GitHub Actions on x86 and
# Travis CI on Arm64.

set -xeuf -o pipefail

# Clippy needs the build to work, the include directory need to be available.
if [ ! -d "mbedtls" ]
then
	git clone https://github.com/ARMmbed/mbedtls.git
fi
pushd mbedtls
git checkout v3.0.0
popd

#################
# Static checks #
#################
# On native target clippy or fmt might not be available.
if cargo fmt -h; then
	cargo fmt --all -- --check
fi
if cargo clippy -h; then
	MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo clippy --all-targets -- -D clippy::all -D clippy::cargo
fi

###########
# C Tests #
###########

cp /tmp/NVChip .
# Start and configure TPM server
tpm_server &
sleep 5
# Ownership has already been taken with "tpm_pass".
tpm2_startup -T mssim

# Create the Parsec socket directory. This must be the default one.
mkdir /run/parsec

# Install and run Parsec
git clone --branch 0.6.0 https://github.com/parallaxsecond/parsec
pushd parsec
cargo build --features tpm-provider --release
./target/release/parsec -c ../ci/config.toml &
sleep 5
popd

# Compile Mbed Crypto for the test application
pushd mbedtls
./scripts/config.py crypto
./scripts/config.py set MBEDTLS_PSA_CRYPTO_SE_C
SHARED=1 make
popd

# Build the driver, clean before to force dynamic linking
cargo clean
MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo build --release

# Compile and run the C application
make -C ci/c-tests run MBED_TLS_PATH=$(pwd)/mbedtls

# Check that Parsec was called by checking if the service contains the key
# this is done by checking if the mappings folder is empty.
# Maybe use parsec-tool instead?
[ "$(ls -A /tmp/mappings)" ]

# Kill Parsec for clean logs
pkill parsec
