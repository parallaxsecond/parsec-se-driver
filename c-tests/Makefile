# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

default: run

main.o:
	gcc -Wall -c -I../include -I../$(MBED_TLS_PATH)/include main.c

test-app: main.o
	gcc -Wall -o test-app main.o -L../$(MBED_TLS_PATH)/library -L../target/debug -l:libparsec_tpm_direct_se_driver.a -l:libmbedcrypto.a -lm -pthread -ldl

run: test-app
	./test-app

clean:
	rm -rf test-app main.o
	rm -rf *.psa_its
