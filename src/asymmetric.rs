// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::psa_se_driver_bindings::{
    psa_algorithm_t, psa_drv_se_asymmetric_t, psa_drv_se_context_t, psa_key_slot_number_t,
    psa_status_t, size_t,
};
use crate::PARSEC_BASIC_CLIENT;
use parsec_client::core::interface::operations::psa_algorithm::{AsymmetricSignature, Hash};

pub(super) static METHODS: psa_drv_se_asymmetric_t = psa_drv_se_asymmetric_t {
    p_sign: Some(p_sign),
    p_verify: Some(p_verify),
    p_encrypt: None,
    p_decrypt: None,
};

fn test_alg() -> AsymmetricSignature {
    AsymmetricSignature::RsaPkcs1v15Sign {
        hash_alg: Hash::Sha256,
    }
}

unsafe extern "C" fn p_sign(
    _drv_context: *mut psa_drv_se_context_t,
    _key_slot: psa_key_slot_number_t,
    _alg: psa_algorithm_t,
    p_hash: *const u8,
    hash_length: size_t,
    p_signature: *mut u8,
    _signature_size: size_t,
    p_signature_length: *mut size_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    let hash = std::slice::from_raw_parts(p_hash, hash_length as usize).to_vec();
    let signature = PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_sign_hash(key_name, hash, test_alg())
        .unwrap();
    let slice: &mut [u8] = std::slice::from_raw_parts_mut(p_signature, signature.len());
    slice.copy_from_slice(&signature);
    *p_signature_length = signature.len() as size_t;
    0
}

unsafe extern "C" fn p_verify(
    _drv_context: *mut psa_drv_se_context_t,
    _key_slot: psa_key_slot_number_t,
    _alg: psa_algorithm_t,
    p_hash: *const u8,
    hash_length: size_t,
    p_signature: *const u8,
    signature_length: size_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    let hash = std::slice::from_raw_parts(p_hash, hash_length as usize).to_vec();
    let signature = std::slice::from_raw_parts(p_signature, signature_length as usize).to_vec();
    PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_verify_hash(key_name, hash, test_alg(), signature)
        .unwrap();
    0
}
