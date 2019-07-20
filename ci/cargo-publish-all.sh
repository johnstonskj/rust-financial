#!/usr/bin/env bash

if [[ "$CRATES" = "" ]] ; then
    echo "Error: no CRATES environment variable"
    exit 1
else
    echo $CRATES
fi

if [[ "$CARGO_TOKEN" = "" ]] ; then
    echo "Error: no $CARGO_TOKEN environment variable"
    exit 2
else
    echo $CARGO_TOKEN
fi

for CRATE in ${CRATES//,/ }
do
    cargo publish --verbose --token $CARGO_TOKEN --manifest-path $CRATE/Cargo.toml
done
