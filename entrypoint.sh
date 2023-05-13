#!/bin/bash

set -e

SOLANA_LISTEN_ADDRESS=${SOLANA_LISTEN_ADDRESS:-http://localhost:8899}

solana config set --url ${SOLANA_LISTEN_ADDRESS}

exec /usr/local/solana/bin/solana-test-validator \
    "$@"
