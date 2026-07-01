#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'; GREEN='\033[0;32m';
fatal() { >&2 echo -e "$RED[Fatal]$NC $1"; exit ${2:-1}; }

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd $SCRIPT_DIR
PLATFORM_DIR="platform/${PLATFORM}"

[ -z ${PLATFORM:-""} ] && fatal "Missing PLATFORM"
[ ! -d "app/${PLATFORM_DIR}" ] && fatal "$PLATFORM_DIR is not valid directory"

cmake -B app/build -S app                       \
    -G Ninja                                    \
    --toolchain=${PLATFORM_DIR}/toolchain.cmake     \
    -D CMAKE_BUILD_TYPE=MinSizeRel              \
    -D PLATFORM=$PLATFORM

cmake --build app/build

