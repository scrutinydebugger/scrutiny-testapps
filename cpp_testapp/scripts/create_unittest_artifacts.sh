#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_DIR="$( realpath "$SCRIPT_DIR/.." )"

cd "$PROJECT_DIR"

OUTPUT_FOLDER=unittest_outputs
BUILD_DIR=build-unittest-artifacts
rm -rf "$OUTPUT_FOLDER"
mkdir "$OUTPUT_FOLDER"
COMPILED_FILE_PATH="$BUILD_DIR/testapp"

build_testapp()
{
    local dwarf_version=$1
    local c_compiler=$2
    local cxx_compiler=$3

    rm -rf "$BUILD_DIR"
    cmake -GNinja \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_C_COMPILER="$c_compiler" \
        -DCMAKE_CXX_COMPILER="$cxx_compiler" \
        -DCMAKE_CXX_STANDARD=11 \
        -DSCRUTINY_TESTAPP_DWARF_VERSION="$dwarf_version" \
        -S "$PROJECT_DIR" \
        -B "$BUILD_DIR"
    cmake --build "$BUILD_DIR" --target testapp
}

for DWARF_VERSION in 2 3 4; do
    build_testapp "$DWARF_VERSION" gcc g++
    filepath="$OUTPUT_FOLDER/testapp-dwarf-gcc-dwarf${DWARF_VERSION}"
    cp "$COMPILED_FILE_PATH" "$filepath"
    "$SCRIPT_DIR/make_testapp_memdump.sh" "$filepath"

    build_testapp "$DWARF_VERSION" clang clang++
    filepath="$OUTPUT_FOLDER/testapp-dwarf-clang-dwarf${DWARF_VERSION}"
    cp "$COMPILED_FILE_PATH" "$filepath"
    "$SCRIPT_DIR/make_testapp_memdump.sh" "$filepath"
done

rm -rf "$BUILD_DIR"
