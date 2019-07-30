#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ "$CARGO_DEPLOY" = "0" ]] ; then
    # Just in case.
    info "skipping deployment step for now"
    exit 0
fi

if [[ "$CARGO_TOKEN" = "" ]] ; then
    # Ensure this is set as a global environment
    #  variable, *and* as a secure one.
    error "no CARGO_TOKEN environment variable"
    exit 2
fi

# There is no '--all' option on publish :-(
if [[ $CARGO_WORKSPACE = 1 ]] ; then
    # Refresh the Cargo.lock file first
    cargo update
    for CRATE in ${CRATES//,/ }
    do
        # must use locked otherwise it doesn't
        # correctly resolve local versions.
        cargo publish $CARGO_FLAGS --locked --token $CARGO_TOKEN --manifest-path $CRATE/Cargo.toml
    done
else
    cargo publish $CARGO_FLAGS --token $CARGO_TOKEN
fi

