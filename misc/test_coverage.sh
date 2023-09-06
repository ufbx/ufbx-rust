#!/usr/bin/env bash

set -eux

mkdir -p build/coverage
rm -f build/coverage/*.profraw

RUSTFLAGS="-C instrument-coverage" \
LLVM_PROFILE_FILE="build/coverage/default_%m_%p.profraw" \
    cargo test --tests

grcov build/coverage --binary-path target/debug -o build/coverage.lcov
