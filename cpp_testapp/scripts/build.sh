#!/bin/bash
# Builds the testapp in ./build. The library is fetched from GitHub at SCRUTINY_EMBEDDED_GIT_TAG (default: master).
# Extra arguments are passed to the cmake configure step.
set -euo pipefail

PROJECT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." >/dev/null 2>&1 && pwd )"
BUILD_DIR="$PROJECT_DIR/build"

SCRUTINY_EMBEDDED_GIT_TAG=${SCRUTINY_EMBEDDED_GIT_TAG:-master}
CMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE:-Release}

set -x

cmake -GNinja \
    -DCMAKE_BUILD_TYPE="$CMAKE_BUILD_TYPE" \
    -DSCRUTINY_EMBEDDED_GIT_TAG="$SCRUTINY_EMBEDDED_GIT_TAG" \
    "$@" \
    -Wno-dev \
    -S "$PROJECT_DIR" \
    -B "$BUILD_DIR"

nice cmake --build "$BUILD_DIR"
