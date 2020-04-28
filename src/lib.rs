// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! # Parsec Secure Element Driver

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
    missing_docs,
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

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    trivial_casts
)]
#[allow(clippy::all)]
mod psa_se_driver_bindings {
    include!(concat!(env!("OUT_DIR"), "/psa_se_driver_bindings.rs"));
}

use std::ptr;

/// SE Driver implementation which hardcodes the provider (TPM) and the authentication method
/// (direct authentication).
#[no_mangle]
pub static mut PARSEC_TPM_DIRECT_SE_DRIVER: psa_se_driver_bindings::psa_drv_se_t = psa_se_driver_bindings::psa_drv_se_t {
    hal_version: psa_se_driver_bindings::PSA_DRV_SE_HAL_VERSION,
    persistent_data_size: 0,
    p_init: None,
    key_management: ptr::null(),
    mac: ptr::null(),
    cipher: ptr::null(),
    aead: ptr::null(),
    asymmetric: ptr::null(),
    derivation: ptr::null(),
};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
