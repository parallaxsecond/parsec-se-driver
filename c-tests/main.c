// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include "psa/crypto.h"
#include "parsec_se_driver.h"

int main()
{
	psa_status_t status;

	status = psa_register_se_driver(PARSEC_TPM_DIRECT_SE_DRIVER_LIFETIME,
			&PARSEC_TPM_DIRECT_SE_DRIVER);
	if (status != PSA_SUCCESS) {
		printf("Register failed (status = %d)\n", status);
		return 1;
	}

	status = psa_crypto_init();
	if (status != PSA_SUCCESS) {
		printf("Init failed (status = %d)\n", status);
		return 1;
	}

	return 0;
}
