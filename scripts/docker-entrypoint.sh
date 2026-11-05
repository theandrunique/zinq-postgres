#!/bin/sh
set -e

KEY_DIR="${KEYS_DIRECTORY:-keys}"
KEY_PATH="${KEY_DIR}/key.pem"

if [ ! -f "${KEY_PATH}" ]; then
    echo "No signing key found at ${KEY_PATH}, generating a new RSA key"
    mkdir -p "${KEY_DIR}"
    openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out "${KEY_PATH}"
fi

exec "$@"
