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
	psa_key_id_t public_key_id = 2;
	psa_key_handle_t public_key_handle;
	psa_algorithm_t alg;
	uint8_t public_key[PSA_KEY_EXPORT_RSA_PUBLIC_KEY_MAX_SIZE(1024)] = {0};
	//uint8_t public_key[PSA_EXPORT_PUBLIC_KEY_OUTPUT_SIZE(PSA_KEY_TYPE_RSA_KEY_PAIR, 1024U)] = {0};
	size_t public_key_length = 0;
	// "Les carottes sont cuites" hased with SHA256
	uint8_t hash[32] = {0xd8, 0xd2, 0xf7, 0x77, 0x79, 0x76, 0x6d, 0x13, 0x1c, 0x8e, 0x06, 0x06,
		0xde, 0x0d, 0xb1, 0xc1, 0x9b, 0xe0, 0x21, 0xb5, 0xfa, 0x74, 0x83, 0x08, 0x3b, 0xda, 0x5e,
		0xf3, 0x51, 0x32, 0xc7, 0x02};
	uint8_t signature[PSA_SIGNATURE_MAX_SIZE] = {0};
	//uint8_t signature[PSA_SIGN_OUTPUT_SIZE(PSA_KEY_TYPE_RSA_KEY_PAIR, 1024U, alg)] = {0};
	size_t signature_length = 0;

	alg = PSA_ALG_RSA_PKCS1V15_SIGN(PSA_ALG_SHA_256);

	// To be activated, need to be executed inside the TPM Docker container
	status = psa_register_se_driver(PSA_KEY_LIFETIME_GET_LOCATION(PARSEC_TPM_DIRECT_SE_DRIVER_LIFETIME),
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
	psa_set_key_type(&key_pair_attributes, PSA_KEY_TYPE_RSA_KEY_PAIR);
	psa_set_key_bits(&key_pair_attributes, 1024U);

	psa_key_attributes_t public_key_attributes = PSA_KEY_ATTRIBUTES_INIT;
	psa_set_key_id(&public_key_attributes, public_key_id);
	psa_set_key_lifetime(&public_key_attributes, PARSEC_TPM_DIRECT_SE_DRIVER_LIFETIME);
	psa_set_key_usage_flags(&public_key_attributes, PSA_KEY_USAGE_VERIFY_HASH);
	psa_set_key_algorithm(&public_key_attributes, alg);
	psa_set_key_type(&public_key_attributes, PSA_KEY_TYPE_RSA_PUBLIC_KEY);
	psa_set_key_bits(&public_key_attributes, 1024U);

	status = psa_generate_key(&key_pair_attributes, &key_pair_handle);
	if (status != PSA_SUCCESS) {
		printf("Key generation failed (status = %d)\n", status);
		return 1;
	}

	status = psa_export_public_key(key_pair_handle,
			public_key,
			PSA_KEY_EXPORT_RSA_PUBLIC_KEY_MAX_SIZE(1024),
			&public_key_length);
	if (status != PSA_SUCCESS) {
		printf("Exporting public key failed (status = %d)\n", status);
		return 1;
	}

	status = psa_import_key(&public_key_attributes,
			public_key,
			public_key_length,
			&public_key_handle);
	if (status != PSA_SUCCESS) {
		printf("Importing key failed (status = %d)\n", status);
		return 1;
	}

	status = psa_sign_hash(key_pair_handle,
			alg,
			hash,
			32,
			signature,
			//PSA_SIGN_OUTPUT_SIZE(PSA_KEY_TYPE_RSA_KEY_PAIR, 1024U, alg),
			PSA_SIGNATURE_MAX_SIZE,
			&signature_length);
	if (status != PSA_SUCCESS) {
		printf("Signing failed (status = %d)\n", status);
		return 1;
	}

	status = psa_verify_hash(public_key_handle,
			alg,
			hash,
			32,
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

	status = psa_destroy_key(public_key_handle);
	if (status != PSA_SUCCESS) {
		printf("Key destruction failed for Public Key (status = %d)\n", status);
		return 1;
	}

	return 0;
}
