#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"
ELF="app/build/bin/leon3_hello.elf"

if [ ! -f "$ELF" ]; then
    >&2 echo "$ELF not found"
    exit 1
fi

qemu-system-sparc       \
    -M leon3_generic    \
    -nographic          \
    -kernel "$ELF"


