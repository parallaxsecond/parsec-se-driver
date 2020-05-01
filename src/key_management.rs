// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::psa_se_driver_bindings::psa_drv_se_key_management_t;

pub(super) static METHODS: psa_drv_se_key_management_t = psa_drv_se_key_management_t {
    p_allocate: None,
    p_validate_slot_number: None,
    p_import: None,
    p_generate: None,
    p_destroy: None,
    p_export: None,
    p_export_public: None,
};
