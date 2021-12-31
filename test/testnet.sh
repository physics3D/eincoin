#!/usr/bin/env bash

echo "GENERATING KEYS"
cargo-run --release --quiet -- gen-key keyfiles/genesis-key
cargo-run --release --quiet -- gen-key keyfiles/key
echo "DONE"
echo "STARTING TESTNET"
foreman start