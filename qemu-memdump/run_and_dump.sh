#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'; GREEN='\033[0;32m';
fatal() { >&2 echo -e "$RED[Fatal]$NC $1"; exit ${2:-1}; }

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

[ -z ${PLATFORM:-""} ] && fatal "Missing PLATFORM"

ELF="app/build/bin/memdump_testapp_${PLATFORM}.elf"
MONITOR_SOCK="/tmp/qemu-monitor-$$.sock"
DUMP_DIR="${SCRIPT_DIR}/memdump"
FIFO="/tmp/dump_fifo"

if [ ! -f "$ELF" ]; then
    >&2 echo "$ELF not found"
    exit 1
fi

mkdir -p "$DUMP_DIR"

cleanup() {
    rm -f "$MONITOR_SOCK"
}
trap cleanup EXIT

echo "[run] ELF:      $ELF"
echo "[run] Monitor:  $MONITOR_SOCK"
echo "[run] Dump dir: $DUMP_DIR"


rm -f "$FIFO"
mkfifo "$FIFO"

if [ "$PLATFORM" = "leon3" ]; then
    QEMU="qemu-system-sparc"
    CPU="leon3_generic"
else
    fatal "Unsupported platform ${PLATFORM}"
fi

qemu-system-sparc \
    -M leon3_generic \
    -nographic \
    -monitor unix:"$MONITOR_SOCK",server,nowait \
    -kernel "$ELF" > "$FIFO" &

qemu_pid=$!

while IFS= read -r line; do
    if [[ "$line" == *"Ready to dump"* ]]; then
        python3 memdump.py --socket "$MONITOR_SOCK" "$ELF" .data .rodata .bss
        break
    fi
done < "$FIFO"
rm -f "$FIFO"

kill -s SIGTERM $qemu_pid
wait

