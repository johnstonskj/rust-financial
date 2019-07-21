#!/usr/bin/env bash

if $(grep -q "^\[workspace\]$" Cargo.toml) ; then
    export CARGO_WORKSPACE=1
    #
    # This will extract the set of members from the workspace's Cargo.toml
    # file, it will construct a comma separated list in the $CRATES
    # environment variable, these SHOULD be listed in order of dependency
    # in your workspace so that publish steps can work effectively.
    #
    export CRATES=$(cat Cargo.toml \
        | egrep -o '"[^"]+"' \
        | tr '"'  ' ' \
        | tr '\n' ' ' \
        | tr -s '[:space:]' \
        | sed -e 's/^[[:space:]]*//' \
        | sed -e 's/[[:space:]]*$//' \
        | tr ' ' ',' \
    )
    echo "Cargo workspace contains ( $CRATES ) crates"
else
    export CARGO_WORKSPACE=0
fi