#!/usr/bin/env bash

if $(grep -q "^\[workspace\]$" Cargo.toml) ; then
    export CARGO_WORKSPACE=1
    export CRATES=$(cat Cargo.toml | egrep -o '"[^"]+"' | tr '"' ' ' |tr '\n' ' ' | tr -s '[:space:]' | sed -e 's/^[[:space:]]*//' |sed -e 's/[[:space:]]*$//' | tr ' ' ',')
    echo "workspace contains $CRATES"
else
    export CARGO_WORKSPACE=0
fi