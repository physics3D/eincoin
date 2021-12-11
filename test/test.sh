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
new-terminal-cargo "genesis genesis-key.priv genesis-key.pub"

gen-key keyfile
new-terminal-cargo "full-node 127.0.0.1"

gen-key keyfile
new-terminal-cargo "full-node 127.0.0.1 -s33333"



cargo-run balance 127.0.0.1 genesis-key.priv genesis-key.pub
