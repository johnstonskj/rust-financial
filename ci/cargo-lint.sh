#!/usr/bin/env bash

source ci/cargo-config.sh

if [[ "$1" == "--install" ]] ; then
    let "exit_code=0"
    for CMD in ${CARGO_LINTER//,/ }
    do
        case "$CMD" in
        fmt)
            rustup component add rustfmt
            let "exit_code += $?"
            ;;
        clippy)
            rustup component add clippy
            let "exit_code += $?"
            ;;
        *)
            echo >&2 "Warning: unknown command $CMD"
            let "exit_code += 100"
            ;;
        esac
    done
    exit exit_code
else
    let "exit_code=0"
    for CMD in ${CARGO_LINTER//,/ }
    do
        case "$CMD" in
        fmt)
            ci/cargo-command.sh fmt --check $*
            let "exit_code += $?"
            ;;
        clippy)
            ci/cargo-command.sh clippy -D warnings $*
            let "exit_code += $?"
            ;;
        *)
            echo >&2 "Warning: unknown command $CMD"
            let "exit_code += 100"
            ;;
        esac
    done
    exit $exit_code
fi

