#!/bin/env sh
# -*- mode:sh; coding:utf-8-unix; -*-

declare -A TERMINAL_COLOR=(
        [RED]='\033[1;31m'
        [GREEN]='\033[1;32m'
        [YELLOW]='\033[1;33m'
        [BLUE]='\033[1;34m'
        [MAGENDA]='\033[1;35m'
        [CYAN]='\033[1;36m'
        [END]='\033[0m'
)

function log () {
    echo -e "$@"
    return $?
}

function logc () {
    local color=$1
    shift
    log "${TERMINAL_COLOR[$color]}$@${TERMINAL_COLOR[END]}"
    return $?
}

function run_with_trace () {
    logc CYAN $@ # && return 0

    $@
    local ret=$?

    if [ $ret -ne 0 ]; then
        logc RED " ...FALED"
        return $ret
    fi
    logc BLUE " ...OK"

    return 0
}

ACTIVE_TOOLCHAIN=$(rustup show active-toolchain | cut -d'-' -f1)

declare -A TARGET_DIR=(
    [stable]="target/stable"
    [beta]="target/beta"
    [nightly]="target/nightly"
)

function run_cargo() {
    local toolchain=$1
    shift

    if [[ $toolchain == $ACTIVE_TOOLCHAIN \
              || $1 == "fmt" \
              || $1 == "update" \
              || $1 == "clippy" \
              || $1 == "deny" \
              || $1 == "msrv" \
        ]]; then
        run_with_trace "cargo +$toolchain $@" || return $?
    else
        local target_dir="${TARGET_DIR[$toolchain]}"
        mkdir -p $target_dir || return $?
        run_with_trace \
            "cargo +$toolchain $@ --target-dir=$target_dir" || return $?
    fi

    return 0
}
