#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ $CARGO_WORKSPACE = 1 ]] ; then
    WS_FLAGS="--all"
else
    WS_FLAGS=""
fi

CARGO_COMMAND=$1
shift

if [[ $# = 0 ]] ; then
    cargo $CARGO_COMMAND $CARGO_FLAGS $WS_FLAGS
else
    cargo $CARGO_COMMAND $CARGO_FLAGS $WS_FLAGS -- $*
fi