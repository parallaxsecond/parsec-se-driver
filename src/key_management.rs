// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::psa_se_driver_bindings::{
    psa_drv_se_context_t, psa_drv_se_key_management_t, psa_key_attributes_t,
    psa_key_creation_method_t, psa_key_slot_number_t, psa_status_t, size_t,
};
use crate::PARSEC_BASIC_CLIENT;
use parsec_client::core::interface::operations::psa_algorithm::{
    Algorithm, AsymmetricSignature, Hash,
};
use parsec_client::core::interface::operations::psa_key_attributes::{
    KeyAttributes, KeyPolicy, KeyType, UsageFlags,
};

fn test_key_attributes() -> KeyAttributes {
    KeyAttributes {
        key_type: KeyType::RsaKeyPair,
        key_bits: 1024,
        key_policy: KeyPolicy {
            key_usage_flags: UsageFlags {
                sign_hash: true,
                verify_hash: true,
                sign_message: false,
                verify_message: false,
                export: false,
                encrypt: false,
                decrypt: false,
                cache: false,
                copy: false,
                derive: false,
            },
            key_algorithm: Algorithm::AsymmetricSignature(AsymmetricSignature::RsaPkcs1v15Sign {
                hash_alg: Hash::Sha256,
            }),
        },
    }
}

pub(super) static METHODS: psa_drv_se_key_management_t = psa_drv_se_key_management_t {
    p_allocate: Some(p_allocate),
    p_validate_slot_number: Some(p_validate_slot_number),
    p_import: Some(p_import),
    p_generate: Some(p_generate),
    p_destroy: Some(p_destroy),
    p_export: None,
    p_export_public: Some(p_export_public),
};

unsafe extern "C" fn p_allocate(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    _attributes: *const psa_key_attributes_t,
    _method: psa_key_creation_method_t,
    _key_slot: *mut psa_key_slot_number_t,
) -> psa_status_t {
    0
}

unsafe extern "C" fn p_validate_slot_number(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    _attributes: *const psa_key_attributes_t,
    _method: psa_key_creation_method_t,
    _key_slot: psa_key_slot_number_t,
) -> psa_status_t {
    0
}

unsafe extern "C" fn p_import(
    _drv_context: *mut psa_drv_se_context_t,
    _key_slot: psa_key_slot_number_t,
    _attributes: *const psa_key_attributes_t,
    data: *const u8,
    data_length: size_t,
    _bits: *mut size_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    let key_material = std::slice::from_raw_parts(data, data_length as usize).to_vec();
    PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_import_key(key_name, key_material, test_key_attributes())
        .unwrap();
    0
}

unsafe extern "C" fn p_generate(
    _drv_context: *mut psa_drv_se_context_t,
    _key_slot: psa_key_slot_number_t,
    _attributes: *const psa_key_attributes_t,
    _pubkey: *mut u8,
    _pubkey_size: size_t,
    _pubkey_length: *mut size_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_generate_key(key_name, test_key_attributes())
        .unwrap();
    0
}

unsafe extern "C" fn p_destroy(
    _drv_context: *mut psa_drv_se_context_t,
    _persistent_data: *mut ::std::os::raw::c_void,
    _key_slot: psa_key_slot_number_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_destroy_key(key_name)
        .unwrap();
    0
}

unsafe extern "C" fn p_export_public(
    _drv_context: *mut psa_drv_se_context_t,
    _key: psa_key_slot_number_t,
    p_data: *mut u8,
    _data_size: size_t,
    p_data_length: *mut size_t,
) -> psa_status_t {
    let key_name = String::from("TEST_KEY");
    let key_material = PARSEC_BASIC_CLIENT
        .read()
        .unwrap()
        .psa_export_public_key(key_name)
        .unwrap();
    let slice: &mut [u8] = std::slice::from_raw_parts_mut(p_data, key_material.len());
    slice.copy_from_slice(&key_material);
    *p_data_length = key_material.len() as size_t;
    0
}
