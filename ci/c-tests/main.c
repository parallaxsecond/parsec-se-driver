// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

// Small example using the PSA Crypto API.
// The example does the following operations:
// * generate a RSA Key Pair (key 1)
// * export its public part
// * import its public part (key 2)
// * sign a hash with key 1
// * verify the signature with key 2
// * destroy key 1 and key 2

#include <stdio.h>

#include "psa/crypto.h"
#include "parsec_se_driver.h"

int main()
{
	psa_status_t status;
	psa_key_id_t key_pair_id = 1;
	psa_key_handle_t key_pair_handle;
	psa_algorithm_t alg;
	uint8_t public_key[PSA_KEY_EXPORT_ECC_PUBLIC_KEY_MAX_SIZE(256)] = {0};
	size_t public_key_length = 0;
	// "Les carottes sont cuites" hased with SHA256
	uint8_t hash[32] = {0xd8, 0xd2, 0xf7, 0x77, 0x79, 0x76, 0x6d, 0x13, 0x1c, 0x8e, 0x06, 0x06,
		0xde, 0x0d, 0xb1, 0xc1, 0x9b, 0xe0, 0x21, 0xb5, 0xfa, 0x74, 0x83, 0x08, 0x3b, 0xda, 0x5e,
		0xf3, 0x51, 0x32, 0xc7, 0x02};
	uint8_t signature[PSA_SIGNATURE_MAX_SIZE] = {0};
	size_t signature_length = 0;

	alg = PSA_ALG_ECDSA(PSA_ALG_SHA_256);

	// To be activated, need to be executed inside the TPM Docker container
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

	psa_key_attributes_t key_pair_attributes = PSA_KEY_ATTRIBUTES_INIT;
	psa_set_key_id(&key_pair_attributes, key_pair_id);
	psa_set_key_lifetime(&key_pair_attributes, PARSEC_TPM_DIRECT_SE_DRIVER_LIFETIME);
	psa_set_key_usage_flags(&key_pair_attributes, PSA_KEY_USAGE_SIGN_HASH | PSA_KEY_USAGE_VERIFY_HASH);
	psa_set_key_algorithm(&key_pair_attributes, alg);
	psa_set_key_type(&key_pair_attributes, PSA_KEY_TYPE_ECC_KEY_PAIR(PSA_ECC_CURVE_SECP_R1));
	psa_set_key_bits(&key_pair_attributes, 256U);

	status = psa_generate_key(&key_pair_attributes, &key_pair_handle);
	if (status != PSA_SUCCESS) {
		printf("Key generation failed (status = %d)\n", status);
		return 1;
	}

	status = psa_export_public_key(key_pair_handle,
			public_key,
			sizeof(public_key),
			&public_key_length);
	if (status != PSA_SUCCESS) {
		printf("Exporting public key failed (status = %d)\n", status);
		return 1;
	}

	status = psa_sign_hash(key_pair_handle,
			alg,
			hash,
			sizeof(hash),
			signature,
			PSA_SIGNATURE_MAX_SIZE,
			&signature_length);
	if (status != PSA_SUCCESS) {
		printf("Signing failed (status = %d)\n", status);
		return 1;
	}

	status = psa_verify_hash(key_pair_handle,
			alg,
			hash,
			sizeof(hash),
			signature,
			signature_length);
	if (status != PSA_SUCCESS) {
		printf("Verifying failed (status = %d)\n", status);
		return 1;
	}

	status = psa_destroy_key(key_pair_handle);
	if (status != PSA_SUCCESS) {
		printf("Key destruction failed for Key PAIR (status = %d)\n", status);
		return 1;
	}

	return 0;
}
