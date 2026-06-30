#!/usr/bin/env bash
set -euo pipefail

IMAGE="sparc-leon3-toolchain"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd $SCRIPT_DIR

cmake -B app/build -S app                       \
    -G Ninja                                    \
    --toolchain=sparc-elf-toolchain.cmake   \
    -D CMAKE_BUILD_TYPE=MinSizeRel

cmake --build app/build

