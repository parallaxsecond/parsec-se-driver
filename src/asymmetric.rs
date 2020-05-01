// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::psa_se_driver_bindings::psa_drv_se_asymmetric_t;

pub(super) static METHODS: psa_drv_se_asymmetric_t = psa_drv_se_asymmetric_t {
    p_sign: None,
    p_verify: None,
    p_encrypt: None,
    p_decrypt: None,
};
