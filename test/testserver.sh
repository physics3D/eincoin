#!/usr/bin/env bash

function cargo-run {
    cargo run --release --quiet -- $@
}

function gen-key {
   cargo-run gen-key $@
}

echo "BUILDING EINCOIN"

cargo build --release || exit 1

echo "KILLING ALL CONFLICTING EINCOIN"

killall eincoin

echo "GENERATING KEYS"

mkdir keyfiles
gen-key keyfiles/genesis-key
gen-key keyfiles/key

echo "STARTING GENESIS NODE"

cargo-run genesis keyfiles/genesis-key.priv.pem -s33333 &
PID=$!

echo "STARTING SERVER"

cargo-run full-node localhost -s3333 -p33333 &
FG=$!


# kill genesis node
sleep 3
echo "DONE BOOTSTRAPPING NETWORK. KILLING GENESIS NODE"
kill $PID

wait