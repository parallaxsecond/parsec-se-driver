// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

#![deny(
    nonstandard_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    missing_copy_implementations
)]
// This one is hard to avoid.
#![allow(clippy::multiple_crate_versions)]

use curl::easy::Easy;
use std::io::Write;
use std::fs::File;
use std::env;
use std::process::Command;
use std::io::{Error, ErrorKind, Result};

// This is the name of the Mbed TLS tag name.
const MBED_TLS_VERSION: &str = "mbedtls-2.22.0";

// Download from here (parameter is the version)
// https://github.com/ARMmbed/mbedtls/archive/mbedtls-2.22.0.tar.gz
fn main() -> Result<()> {
    let path = env::var("OUT_DIR").unwrap() + "/" + "mbed_tls.tar.gz";
    let mut mbed_tls = File::create(&path)?;
    let mut dst: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.url(&format!("https://github.com/ARMmbed/mbedtls/archive/{}.tar.gz", MBED_TLS_VERSION))?;
    easy.follow_location(true)?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    mbed_tls.write_all(&dst.clone())?;

    let status = Command::new("tar")
            .arg("-C")
            .arg(env::var("OUT_DIR").unwrap())
            .arg("-xf")
            .arg(&path)
            .status()?;
    if !status.success() {
        return Err(Error::new(ErrorKind::Other, "tar command failed"));
    }

    // Name of folder: OUT_DIR/mbedtls-mbedtls-2.22.0

    let psa_include_dir = env::var("OUT_DIR").unwrap() + "/mbedtls-" + MBED_TLS_VERSION + "/include/psa/";
    let header = psa_include_dir.clone() + "crypto_se_driver.h";

    println!("cargo:rerun-if-changed={}", header);

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", psa_include_dir))
        .rustfmt_bindings(true)
        .header(header)
        .generate_comments(false)
        .generate()
        .or_else(|_| {
            Err(Error::new(
                ErrorKind::Other,
                "Unable to generate bindings to mbed crypto",
            ))
        })?;

    println!("OUT_DIR = {:?}", env::var("OUT_DIR").unwrap());

    bindings.write_to_file(env::var("OUT_DIR").unwrap() + "/psa_se_driver_bindings.rs")
}
