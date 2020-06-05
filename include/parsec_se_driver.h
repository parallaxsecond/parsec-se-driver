// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

#ifndef PARSEC_SE_DRIVER_H
#define PARSEC_SE_DRIVER_H

#include "psa/crypto_se_driver.h"

// Parsec SE Driver implementation using the TPM provider and direct authentication.
// This value is compatible with lifetime seen as a combination of location and persistence.
// The location is 0x000001 (primary SE) and persistence is 0x01 (persistent).
#define PARSEC_TPM_DIRECT_SE_DRIVER_LIFETIME ((psa_key_lifetime_t)0x00000101)
extern psa_drv_se_t PARSEC_TPM_DIRECT_SE_DRIVER;

#endif /* PARSEC_SE_DRIVER_H */
