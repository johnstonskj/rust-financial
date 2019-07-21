#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ $CARGO_WORKSPACE = 1 ]] ; then
    WS_FLAGS="--all"
else
    WS_FLAGS=""
fi

if [[ "$1" == "--clean" ]] ; then
    echo "Cleaning up first..."
    cargo clean $CARGO_FLAGS --release --doc
fi

echo "Running build, test, doc..."
cargo build $CARGO_FLAGS $WS_FLAGS && \
cargo test $CARGO_FLAGS $WS_FLAGS && \
cargo doc $CARGO_FLAGS $WS_FLAGS --no-deps
