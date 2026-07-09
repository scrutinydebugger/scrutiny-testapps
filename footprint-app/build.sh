#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'; GREEN='\033[0;32m';
fatal() { >&2 echo -e "$RED[Fatal]$NC $1"; exit ${2:-1}; }

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd $SCRIPT_DIR

cmake -B app/build -S app               \
    -G Ninja                            \
    --toolchain=gcc-arm-none-eabi.cmake \

cmake --build app/build

