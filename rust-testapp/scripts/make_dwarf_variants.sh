#!/bin/bash
# Builds the testapp once per DWARF version and stores binary + memdump in testfiles/ as
# <date>_rust-testapp-rustc_v<rustc version>_dwarf-v<dwarf version>[.memdump]
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_DIR="$( realpath "$SCRIPT_DIR/.." )"
OUTPUT_DIR="$PROJECT_DIR/testfiles"
DWARF_VERSIONS="2 3 4 5"

DATE="$(date +%Y%m%d)"
RUSTC_VERSION="$(rustc --version | awk '{print $2}')"

mkdir -p "$OUTPUT_DIR"
cd "$PROJECT_DIR"

for dwarf_version in $DWARF_VERSIONS; do
    echo "=== DWARF v$dwarf_version"
    target_dir="$PROJECT_DIR/target/dwarf-v$dwarf_version"

    # RUSTFLAGS replaces the rustflags of .cargo/config.toml, so they are repeated here.
    RUSTFLAGS="-C relocation-model=static -C dwarf-version=$dwarf_version" \
        cargo build --target-dir "$target_dir"

    binary="$target_dir/debug/rust-testapp"
    name="${DATE}_rust-testapp-rustc_v${RUSTC_VERSION}_dwarf-v${dwarf_version}"

    cp "$binary" "$OUTPUT_DIR/$name"
    memrange="$(python3 "$SCRIPT_DIR/get_elf_memranges.py" "$OUTPUT_DIR/$name")"
    "$OUTPUT_DIR/$name" memdump $memrange > "$OUTPUT_DIR/$name.memdump"

    echo "Wrote $OUTPUT_DIR/$name"
    echo "Wrote $OUTPUT_DIR/$name.memdump"
done
