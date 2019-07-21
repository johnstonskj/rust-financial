#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ $CARGO_WORKSPACE = 1 ]] ; then
    WS_FLAGS="--all"
else
    WS_FLAGS=""
fi

cargo build $CARGO_FLAGS $WS_FLAGS && \
cargo test $CARGO_FLAGS $WS_FLAGS && \
cargo doc $CARGO_FLAGS $WS_FLAGS --no-deps
