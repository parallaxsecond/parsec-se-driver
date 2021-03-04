<!--
  -- Copyright 2020 Contributors to the Parsec project. 
  -- SPDX-License-Identifier: Apache-2.0
--->
# Parsec Secure Element Driver

This repository contains an implementation of a PSA Secure Element using the [Parsec service](https://github.com/parallaxsecond/parsec).
It implements the Secure Element Hardware Abstraction Layer and compiles to a library exposing
a `psa_drv_se_t` structure.

## How to build and use the driver

When being built, this driver needs to use the same PSA Crypto API interface
implementation that is going to register it.  You need to specify the location
of `psa/` header files folder with the environment variable
`MBEDTLS_INCLUDE_DIR`. For example if the `mbedtls` project is on the same
directory:

```bash
$ MBEDTLS_INCLUDE_DIR=$(pwd)/mbedtls/include cargo build
```

This will produce `libparsec_se_driver.a` (and `.so`) in
`target/debug` or `target/release`.  This library contains the `psa_drv_se_t`
symbol defined in the `include/parsec_se_driver.h` file.  That header file
should be included under the same include directory than the PSA `psa/crypto.h`
file coming from the PSA Cryptography API implementation.

By default, this SE Driver will make its requests using the Parsec Provider
with the highest priority. If you would like it to always select a specific provider,
use the `pkcs11` feature for the PKCS11 provider or the `tpm` feature for the TPM provider.
If both features are enabled, the `tpm` one will take precedence.

The build scripts have a dependency on `libclang`, which is needed on the
system.

The driver has been tested with Mbed Crypto from the GitHub Mbed TLS repository version
2.22.0 and 2.25.0. Mbed TLS 2.25.0 fixes BDSA-2020-2270 (CVE-2020-16150). You should always
use the latest Mbed TLS release officially available.

The current version of this SE driver (`0.4.0`) is compatible with Parsec version `0.6.0`.
For compatibility of older versions, check which Parsec service version was used in the `ci/ci.sh`
script for those SE driver versions.

## Notice

This implementation is currently work-in-progress and might not implement all operations or
parameters of the HAL.

The driver produced currently uses Parsec default authentication method.
If Parsec is using the Direct authenticator, the
application name of requests made to Parsec by this SE driver will be "Parsec
SE Driver".

Make sure to check the
[Parsec](https://parallaxsecond.github.io/parsec-book/threat_model/threat_model.html)
and [Parsec Rust
Client](https://parallaxsecond.github.io/parsec-book/threat_model/rust_client_threat_model.html)
threat models to make sure that your use-case is secure.

## License

The software is provided under Apache-2.0. Contributions to this project are accepted under the same license.

## Contributing

Please check the [**Contribution Guidelines**](https://parallaxsecond.github.io/parsec-book/contributing.html)
to know more about the contribution process.
