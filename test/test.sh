#!/usr/bin/env bash

function cargo-run {
    cargo run --release --quiet -- $@
}

function gen-key {
   cargo-run gen-key $@
}

function new-terminal-cargo {
    terminator -e "cargo run --release --quiet -- $1; zsh"
}

cargo build --release || exit 1

mkdir keyfiles
gen-key keyfiles/genesis-key
gen-key keyfiles/key

new-terminal-cargo "genesis keyfiles/genesis-key.priv.pem"

new-terminal-cargo "full-node 127.0.0.1 -m -k keyfiles/key.priv.pem"

new-terminal-cargo "full-node 127.0.0.1 -s33333"



cargo-run -lWARN balance 127.0.0.1 keyfiles/genesis-key.priv.pem
cargo-run -lWARN balance localhost -p33333 keyfiles/key.priv.pem

echo

cargo-run transaction localhost 35 keyfiles/key.pub.pem keyfiles/genesis-key.priv.pem

echo

cargo-run -lWARN balance 127.0.0.1 -p33333 keyfiles/genesis-key.priv.pem
cargo-run -lWARN balance localhost keyfiles/key.priv.pem
