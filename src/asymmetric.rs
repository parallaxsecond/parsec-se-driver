// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::{client_error_to_psa_status, key_slot_to_key_name, PARSEC_BASIC_CLIENT};
use psa_crypto::ffi::{
    psa_algorithm_t, psa_drv_se_asymmetric_t, psa_drv_se_context_t, psa_key_slot_number_t,
    psa_status_t, PSA_SUCCESS,
};
use psa_crypto::types::algorithm::AsymmetricSignature;
use std::convert::TryFrom;

pub(super) static METHODS: psa_drv_se_asymmetric_t = psa_drv_se_asymmetric_t {
    private_p_sign: Some(p_sign),
    private_p_verify: Some(p_verify),
    private_p_encrypt: None,
    private_p_decrypt: None,
};

unsafe extern "C" fn p_sign(
    _drv_context: *mut psa_drv_se_context_t,
    key_slot: psa_key_slot_number_t,
    alg: psa_algorithm_t,
    p_hash: *const u8,
    hash_length: usize,
    p_signature: *mut u8,
    _signature_size: usize,
    p_signature_length: *mut usize,
) -> psa_status_t {
    let alg = match AsymmetricSignature::try_from(alg) {
        Ok(alg) => alg,
        Err(e) => return e.into(),
    };
    let signature = match PARSEC_BASIC_CLIENT.read().unwrap().psa_sign_hash(
        &key_slot_to_key_name(key_slot),
        std::slice::from_raw_parts(p_hash, hash_length),
        alg,
    ) {
        Ok(signature) => signature,
        Err(e) => return client_error_to_psa_status(e),
    };
    let slice: &mut [u8] = std::slice::from_raw_parts_mut(p_signature, signature.len());
    slice.copy_from_slice(&signature);
    *p_signature_length = signature.len();

    PSA_SUCCESS
}

unsafe extern "C" fn p_verify(
    _drv_context: *mut psa_drv_se_context_t,
    key_slot: psa_key_slot_number_t,
    alg: psa_algorithm_t,
    p_hash: *const u8,
    hash_length: usize,
    p_signature: *const u8,
    signature_length: usize,
) -> psa_status_t {
    let alg = match AsymmetricSignature::try_from(alg) {
        Ok(alg) => alg,
        Err(e) => return e.into(),
    };
    match PARSEC_BASIC_CLIENT.read().unwrap().psa_verify_hash(
        &key_slot_to_key_name(key_slot),
        std::slice::from_raw_parts(p_hash, hash_length),
        alg,
        std::slice::from_raw_parts(p_signature, signature_length),
    ) {
        Ok(_) => PSA_SUCCESS,
        Err(e) => client_error_to_psa_status(e),
    }
}
