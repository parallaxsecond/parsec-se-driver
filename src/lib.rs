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
    //trivial_casts,
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
mod asymmetric;
mod key_management;

use psa_se_driver_bindings::{
    psa_drv_se_asymmetric_t, psa_drv_se_context_t, psa_drv_se_key_management_t, psa_drv_se_t,
    psa_key_lifetime_t, psa_status_t, PSA_DRV_SE_HAL_VERSION,
};

use lazy_static::lazy_static;
use parsec_client::auth::AuthenticationData;
use parsec_client::BasicClient;
use std::ptr;
use std::sync::RwLock;
use uuid::Uuid;

lazy_static! {
    static ref PARSEC_BASIC_CLIENT: RwLock<BasicClient> = {
        let app_auth_data = AuthenticationData::AppIdentity(String::from("Parsec SE Driver"));
        let client = BasicClient::new(app_auth_data);

        RwLock::new(client)
    };
}

/// SE Driver implementation which hardcodes the provider (TPM) and the authentication method
/// (direct authentication).
#[no_mangle]
pub static mut PARSEC_TPM_DIRECT_SE_DRIVER: psa_drv_se_t = psa_drv_se_t {
    hal_version: PSA_DRV_SE_HAL_VERSION,
    persistent_data_size: 0,
    p_init: Some(p_init),
    key_management: &key_management::METHODS as *const psa_drv_se_key_management_t,
    mac: ptr::null(),
    cipher: ptr::null(),
    aead: ptr::null(),
    asymmetric: &asymmetric::METHODS as *const psa_drv_se_asymmetric_t,
    derivation: ptr::null(),
};

unsafe extern "C" fn p_init(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    _lifetime: psa_key_lifetime_t,
) -> psa_status_t {
    let mut client = (*PARSEC_BASIC_CLIENT).write().expect("lock poisoned");

    let providers = match client.list_providers() {
        Ok(providers) => providers,
        Err(e) => {
            eprintln!("error getting available providers: {:?}.", e);
            return 1;
        }
    };
    let tpm_provider_uuid = Uuid::parse_str("1e4954a4-ff21-46d3-ab0c-661eeb667e1d").unwrap();
    let provider_id = match providers
        .iter()
        .filter(|p| p.uuid == tpm_provider_uuid)
        .last()
    {
        Some(provider) => provider.id,
        None => {
            eprintln!("TPM provider not registered in the Parsec service.");
            return 1;
        }
    };

    client.set_implicit_provider(provider_id);

    0
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
