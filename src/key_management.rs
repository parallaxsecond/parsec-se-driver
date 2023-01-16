// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::{client_error_to_psa_status, key_slot_to_key_name, PARSEC_BASIC_CLIENT};
use psa_crypto::ffi::{
    psa_drv_se_context_t, psa_drv_se_key_management_t, psa_get_key_id, psa_key_attributes_t,
    psa_key_creation_method_t, psa_key_slot_number_t, psa_status_t, PSA_SUCCESS,
};
use psa_crypto::types::key::Attributes;
use std::convert::TryFrom;

pub(super) static METHODS: psa_drv_se_key_management_t = psa_drv_se_key_management_t {
    private_p_allocate: Some(p_allocate),
    private_p_validate_slot_number: Some(p_validate_slot_number),
    private_p_import: Some(p_import),
    private_p_generate: Some(p_generate),
    private_p_destroy: Some(p_destroy),
    private_p_export: None,
    private_p_export_public: Some(p_export_public),
};

unsafe extern "C" fn p_allocate(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    attributes: *const psa_key_attributes_t,
    _method: psa_key_creation_method_t,
    key_slot: *mut psa_key_slot_number_t,
) -> psa_status_t {
    *key_slot = psa_get_key_id(attributes).into();
    PSA_SUCCESS
}

unsafe extern "C" fn p_validate_slot_number(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    _attributes: *const psa_key_attributes_t,
    _method: psa_key_creation_method_t,
    _key_slot: psa_key_slot_number_t,
) -> psa_status_t {
    PSA_SUCCESS
}

unsafe extern "C" fn p_import(
    _drv_context: *mut psa_drv_se_context_t,
    key_slot: psa_key_slot_number_t,
    attributes: *const psa_key_attributes_t,
    data: *const u8,
    data_length: usize,
    bits: *mut usize,
) -> psa_status_t {
    let attributes = match Attributes::try_from(*attributes) {
        Ok(alg) => alg,
        Err(e) => return e.into(),
    };
    *bits = attributes.bits;
    match PARSEC_BASIC_CLIENT.read().unwrap().psa_import_key(
        &key_slot_to_key_name(key_slot),
        std::slice::from_raw_parts(data, data_length),
        attributes,
    ) {
        Ok(_) => PSA_SUCCESS,
        Err(e) => client_error_to_psa_status(e),
    }
}

unsafe extern "C" fn p_generate(
    _drv_context: *mut psa_drv_se_context_t,
    key_slot: psa_key_slot_number_t,
    attributes: *const psa_key_attributes_t,
    _pubkey: *mut u8,
    _pubkey_size: usize,
    _pubkey_length: *mut usize,
) -> psa_status_t {
    let attributes = match Attributes::try_from(*attributes) {
        Ok(alg) => alg,
        Err(e) => return e.into(),
    };
    match PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_generate_key(&key_slot_to_key_name(key_slot), attributes)
    {
        Ok(_) => PSA_SUCCESS,
        Err(e) => client_error_to_psa_status(e),
    }
}

unsafe extern "C" fn p_destroy(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    key_slot: psa_key_slot_number_t,
) -> psa_status_t {
    match PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_destroy_key(&key_slot_to_key_name(key_slot))
    {
        Ok(_) => PSA_SUCCESS,
        Err(e) => client_error_to_psa_status(e),
    }
}

unsafe extern "C" fn p_export_public(
    _drv_context: *mut psa_drv_se_context_t,
    key: psa_key_slot_number_t,
    p_data: *mut u8,
    _data_size: usize,
    p_data_length: *mut usize,
) -> psa_status_t {
    let key_material = match PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_export_public_key(&key_slot_to_key_name(key))
    {
        Ok(key) => key,
        Err(e) => return client_error_to_psa_status(e),
    };
    let slice: &mut [u8] = std::slice::from_raw_parts_mut(p_data, key_material.len());
    slice.copy_from_slice(&key_material);
    *p_data_length = key_material.len();

    PSA_SUCCESS
}
