#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ "$CARGO_TOKEN" = "" ]] ; then
    echo "Error: no CARGO_TOKEN environment variable"
    exit 2
else
    echo $CARGO_TOKEN
fi

if [[ $CARGO_WORKSPACE = 1 ]] ; then
    for CRATE in ${CRATES//,/ }
    do
        echo cargo publish $CARGO_FLAGS --token $CARGO_TOKEN --manifest-path $CRATE/Cargo.toml
    done
else
    cargo publish $CARGO_FLAGS --token $CARGO_TOKEN
fi

