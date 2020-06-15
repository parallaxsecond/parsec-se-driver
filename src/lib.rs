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

mod asymmetric;
mod key_management;

use psa_crypto::ffi::{
    psa_drv_se_asymmetric_t, psa_drv_se_context_t, psa_drv_se_key_management_t, psa_drv_se_t,
    psa_key_lifetime_t, psa_key_slot_number_t, psa_status_t, PSA_DRV_SE_HAL_VERSION,
};
use psa_crypto::ffi::{
    PSA_ERROR_ALREADY_EXISTS,
    PSA_ERROR_BAD_STATE,
    PSA_ERROR_BUFFER_TOO_SMALL,
    PSA_ERROR_COMMUNICATION_FAILURE,
    PSA_ERROR_DOES_NOT_EXIST,
    PSA_ERROR_GENERIC_ERROR,
    //PSA_ERROR_DATA_CORRUPT,
    //PSA_ERROR_DATA_INVALID,
    PSA_ERROR_HARDWARE_FAILURE,
    PSA_ERROR_INSUFFICIENT_DATA,
    //PSA_ERROR_CORRUPTION_DETECTED,
    PSA_ERROR_INSUFFICIENT_ENTROPY,
    PSA_ERROR_INSUFFICIENT_MEMORY,
    PSA_ERROR_INSUFFICIENT_STORAGE,
    PSA_ERROR_INVALID_ARGUMENT,
    PSA_ERROR_INVALID_HANDLE,
    PSA_ERROR_INVALID_PADDING,
    PSA_ERROR_INVALID_SIGNATURE,
    PSA_ERROR_NOT_PERMITTED,
    PSA_ERROR_NOT_SUPPORTED,
    PSA_ERROR_STORAGE_FAILURE,
    PSA_SUCCESS,
};

use lazy_static::lazy_static;
use log::error;
use parsec_client::auth::AuthenticationData;
use parsec_client::core::interface::requests::ResponseStatus;
use parsec_client::error::Error;
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
    _location: psa_key_lifetime_t,
) -> psa_status_t {
    let mut client = (*PARSEC_BASIC_CLIENT).write().expect("lock poisoned");

    #[cfg(logging)]
    env_logger::init();

    log::info!("SE Driver initialization");

    let providers = match client.list_providers() {
        Ok(providers) => providers,
        Err(e) => {
            error!("error getting available providers: {:?}.", e);
            return PSA_ERROR_GENERIC_ERROR;
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
            error!("TPM provider not registered in the Parsec service.");
            return PSA_ERROR_GENERIC_ERROR;
        }
    };

    client.set_implicit_provider(provider_id);

    PSA_SUCCESS
}

fn key_slot_to_key_name(key_slot: psa_key_slot_number_t) -> String {
    format!("parsec-se-driver-key{}", key_slot)
}

fn client_error_to_psa_status(error: Error) -> psa_status_t {
    match error {
        Error::Service(ResponseStatus::Success) => PSA_SUCCESS,
        Error::Service(ResponseStatus::PsaErrorGenericError) => PSA_ERROR_GENERIC_ERROR,
        Error::Service(ResponseStatus::PsaErrorNotSupported) => PSA_ERROR_NOT_SUPPORTED,
        Error::Service(ResponseStatus::PsaErrorNotPermitted) => PSA_ERROR_NOT_PERMITTED,
        Error::Service(ResponseStatus::PsaErrorBufferTooSmall) => PSA_ERROR_BUFFER_TOO_SMALL,
        Error::Service(ResponseStatus::PsaErrorAlreadyExists) => PSA_ERROR_ALREADY_EXISTS,
        Error::Service(ResponseStatus::PsaErrorDoesNotExist) => PSA_ERROR_DOES_NOT_EXIST,
        Error::Service(ResponseStatus::PsaErrorBadState) => PSA_ERROR_BAD_STATE,
        Error::Service(ResponseStatus::PsaErrorInvalidArgument) => PSA_ERROR_INVALID_ARGUMENT,
        Error::Service(ResponseStatus::PsaErrorInsufficientMemory) => PSA_ERROR_INSUFFICIENT_MEMORY,
        Error::Service(ResponseStatus::PsaErrorInsufficientStorage) => {
            PSA_ERROR_INSUFFICIENT_STORAGE
        }
        Error::Service(ResponseStatus::PsaErrorCommunicationFailure) => {
            PSA_ERROR_COMMUNICATION_FAILURE
        }
        Error::Service(ResponseStatus::PsaErrorStorageFailure) => PSA_ERROR_STORAGE_FAILURE,
        //Error::Service(ResponseStatus::PsaErrorDataCorrupt) => PSA_ERROR_DATA_CORRUPT,
        //Error::Service(ResponseStatus::PsaErrorDataInvalid) => PSA_ERROR_DATA_INVALID,
        Error::Service(ResponseStatus::PsaErrorHardwareFailure) => PSA_ERROR_HARDWARE_FAILURE,
        //Error::Service(ResponseStatus::PsaErrorCorruptionDetected) => PSA_ERROR_CORRUPTION_DETECTED,
        Error::Service(ResponseStatus::PsaErrorInsufficientEntropy) => {
            PSA_ERROR_INSUFFICIENT_ENTROPY
        }
        Error::Service(ResponseStatus::PsaErrorInvalidSignature) => PSA_ERROR_INVALID_SIGNATURE,
        Error::Service(ResponseStatus::PsaErrorInvalidPadding) => PSA_ERROR_INVALID_PADDING,
        Error::Service(ResponseStatus::PsaErrorInsufficientData) => PSA_ERROR_INSUFFICIENT_DATA,
        Error::Service(ResponseStatus::PsaErrorInvalidHandle) => PSA_ERROR_INVALID_HANDLE,
        e => {
            error!("Conversion of {:?} to PSA_ERROR_GENERIC_ERROR.", e);
            PSA_ERROR_GENERIC_ERROR
        }
    }
}
