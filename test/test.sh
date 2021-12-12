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

gen-key genesis-key
gen-key key

new-terminal-cargo "genesis genesis-key.priv.pem"

new-terminal-cargo "full-node 127.0.0.1 -m -k key.priv.pem"

new-terminal-cargo "full-node 127.0.0.1 -s33333"



cargo-run -lWARN balance 127.0.0.1 genesis-key.priv.pem
cargo-run -lWARN balance localhost -p33333 key.priv.pem

echo

cargo-run transaction localhost 35 key.pub.pem genesis-key.priv.pem

echo

cargo-run -lWARN balance 127.0.0.1 -p33333 genesis-key.priv.pem
cargo-run -lWARN balance localhost key.priv.pem
