// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

#ifndef PARSEC_SE_DRIVER_H
#define PARSEC_SE_DRIVER_H

#include "psa/crypto_se_driver.h"

// Parsec SE Driver implementation using the TPM provider and direct authentication.
extern psa_drv_se_t PARSEC_TPM_DIRECT_SE_DRIVER;

#endif /* PARSEC_SE_DRIVER_H */
