#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ $# -lt 1 ]] ; then
    error "no CARGO_COMMAND argument supplied"
    exit 1
fi

if [[ $CARGO_WORKSPACE = 1 ]] ; then
    WS_FLAGS="--all"
else
    WS_FLAGS=""
fi

CARGO_COMMAND=$1
shift

debug "running $CARGO_COMMAND"

if [[ $# = 0 ]] ; then
    cargo $CARGO_COMMAND $CARGO_FLAGS $WS_FLAGS
else
    cargo $CARGO_COMMAND $CARGO_FLAGS $WS_FLAGS -- $*
fi