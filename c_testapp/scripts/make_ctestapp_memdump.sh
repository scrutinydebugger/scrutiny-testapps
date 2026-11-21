#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
default_file="$SCRIPT_DIR/../build/c_testapp"
file="$(realpath "${1:-$default_file}")"

MEMRANGE="$(python3 "$SCRIPT_DIR/get_elf_memranges.py" "$file")"
"$file" memdump $MEMRANGE > "$file.memdump"
echo "Memdump written to $file.memdump"
