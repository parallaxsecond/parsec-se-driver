<!--
  -- Copyright 2020 Contributors to the Parsec project. 
  -- SPDX-License-Identifier: Apache-2.0
--->
# Parsec Secure Element Driver

This repository contains an implementation of a PSA Secure Element using the [Parsec service](https://github.com/parallaxsecond/parsec).
It implements the Secure Element Hardware Abstraction Layer and compiles to a library exposing
a `psa_drv_se_t` structure.

## How to build and use the driver

When being built, this driver needs to dynamically link with your PSA Crypto
API implementation that is going to register it.  You need to specify the
location of libmbedcrypto.so and the `psa/` header files folder with the
environment variable `MBEDTLS_LIB_DIR` and `MBEDTLS_INCLUDE_DIR`. For example
if the `mbedtls` project is on the same directory:

```bash
$ MBEDTLS_LIB_DIR=$(pwd)/mbedtls/library MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo build
```

This will produce `libparsec_tpm_direct_se_driver.a` (and `.so`) in
`target/debug` or `target/release`.  This library contains the `psa_drv_se_t`
symbol defined in the `include/parsec_se_driver.h` file.  That header file
should be included under the same include directory than the PSA `psa/crypto.h`
file coming from the PSA Cryptography API implementation.

The build scripts have a dependency on `libclang`, which is needed on the
system.

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
