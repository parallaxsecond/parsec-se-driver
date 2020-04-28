<!--
  -- Copyright 2020 Contributors to the Parsec project. 
  -- SPDX-License-Identifier: Apache-2.0
--->
# Parsec Secure Element Driver

This repository contains an implementation of a PSA Secure Element using the [Parsec service](https://github.com/parallaxsecond/parsec).
It implements the Secure Element Hardware Abstraction Layer and compiles to a library exposing
a `psa_drv_se_t` structure.

## How to build and use the driver

To build you need `tar` and `libclang`:
```bash
$ cargo build
```
will produce `libparsec_tpm_direct_se_driver.a` (and `.so`) in `target/debug` or `target/release`.
This library contains the `psa_drv_se_t` symbol defined in the `include/parsec_se_driver.h` file.
That header file should be included under the same include directory than the PSA `psa/crypto.h`
file coming from the PSA Cryptography API implementation.

## Notice

This implementation is currently work-in-progress and might not implement all operations or
parameters of the HAL.

The driver produced currently hardcodes the following parameters:
* it uses the TPM provider
* it uses direct authentication
* it expects the Mbed Crypto implementation from Mbed TLS version `MBED_TLS_VERSION` in `build.rs`

## License

The software is provided under Apache-2.0. Contributions to this project are accepted under the same license.

## Contributing

Please check the [**Contribution Guidelines**](https://parallaxsecond.github.io/parsec-book/contributing.html)
to know more about the contribution process.
