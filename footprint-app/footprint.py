#!/usr/bin/env python3
"""
Analyze a GNU linker map file.

  map-footprint.py lib <map_file> <pattern> [-d|--details]
      Sum .text/.rodata/.bss contributions from objects whose path contains <pattern>.

  map-footprint.py sym <map_file> <pattern>
      Find all symbols whose name (demangled or mangled) contains <pattern>
      and print their size and location.
"""

import argparse
import re
import sys
from pathlib import Path

TRACKED = {'.text', '.rodata', '.bss'}

# Non-indented line that looks like an output section header.
_OUTPUT_SECTION = re.compile(r'^(\.[^\s]+)\s+0x[0-9a-fA-F]+\s+0x[0-9a-fA-F]+')

# Indented line with size (0x...) and a path — covers both:
#   .subsect  0xADDR  0xSIZE  path   (one-line form)
#              0xADDR  0xSIZE  path   (continuation)
# The optional leading token is the subsection name when present.
_SIZE_LINE = re.compile(
    r'^\s+(\S+\s+)?0x[0-9a-fA-F]+\s+(0x[0-9a-fA-F]+)\s+(\S+)\s*$'
)

# Indented line containing only a section/subsection name (two-line format).
_SECTION_ONLY = re.compile(r'^\s+(\.[^\s]+)\s*$')


def _is_symbol_line(line: str) -> tuple[bool, str]:
    """
    Detect symbol-address lines:  0xADDR   symbol_name (possibly with spaces).
    Returns (True, name) or (False, '').
    """
    stripped = line.strip()
    if not stripped.startswith('0x'):
        return False, ''
    first, _, rest = stripped.partition(' ')
    rest = rest.lstrip()
    # If the remainder starts with 0x it's a size/addr field, not a name.
    if not rest or rest.startswith('0x'):
        return False, ''
    return True, rest.rstrip()


def parse_entries(map_file: str) -> list[dict]:
    """
    Parse the 'Linker script and memory map' section and return one dict per
    input-section contribution inside .text / .rodata / .bss.

    Each dict has: section, subsection, size, path, names (demangled).
    """
    entries: list[dict] = []
    in_map = False
    current_section: str | None = None
    pending: dict | None = None   # entry whose size line hasn't arrived yet
    current_entry: dict | None = None  # last fully-built entry (for name lines)

    with open(map_file, errors='replace') as f:
        for line in f:
            if not in_map:
                if 'Linker script and memory map' in line:
                    in_map = True
                continue

            # --- Output-section header (not indented) ---
            if line and not line[0].isspace():
                m = _OUTPUT_SECTION.match(line)
                current_section = m.group(1) if (m and m.group(1) in TRACKED) else None
                pending = None
                continue

            if current_section is None:
                continue

            # --- Size + path line ---
            m = _SIZE_LINE.match(line)
            if m:
                size = int(m.group(2), 16)
                path = m.group(3)

                if pending is not None:
                    # Two-line form: complete the pending entry.
                    pending['size'] = size
                    pending['path'] = path
                    if size > 0:
                        entries.append(pending)
                        current_entry = pending
                    pending = None
                else:
                    # One-line form: extract subsection name from the leading token.
                    leading = m.group(1)
                    subsection = leading.strip() if leading else None
                    if size > 0:
                        entry = {
                            'section': current_section,
                            'subsection': subsection,
                            'size': size,
                            'path': path,
                            'names': [],
                        }
                        entries.append(entry)
                        current_entry = entry
                continue

            # --- Section-name-only line (first line of two-line form) ---
            m = _SECTION_ONLY.match(line)
            if m:
                pending = {
                    'section': current_section,
                    'subsection': m.group(1),
                    'size': 0,
                    'path': None,
                    'names': [],
                }
                continue

            # --- Symbol-address line ---
            ok, name = _is_symbol_line(line)
            if ok and current_entry is not None:
                current_entry['names'].append(name)

    return entries


# ---------------------------------------------------------------------------
# lib mode
# ---------------------------------------------------------------------------

def cmd_lib(args: argparse.Namespace) -> None:
    entries = parse_entries(args.map_file)
    pattern = args.library

    totals = {s: 0 for s in TRACKED}
    details: dict[str, list[dict]] = {s: [] for s in TRACKED}

    for e in entries:
        if e['path'] and pattern in e['path']:
            totals[e['section']] += e['size']
            details[e['section']].append(e)

    grand = sum(totals.values())
    print(f"Map file : {args.map_file}")
    print(f"Pattern  : {pattern}")
    print()
    print(f"{'Section':<12} {'Bytes':>10}  {'Hex':>10}")
    print("-" * 38)
    for section in ('.text', '.rodata', '.bss'):
        n = totals[section]
        print(f"{section:<12} {n:>10,}  {n:#010x}")
    print("-" * 38)
    print(f"{'TOTAL':<12} {grand:>10,}  {grand:#010x}")

    if args.details:
        for section in ('.text', '.rodata', '.bss'):
            group = details[section]
            if not group:
                continue
            by_path: dict[str, int] = {}
            for e in group:
                by_path[e['path']] = by_path.get(e['path'], 0) + e['size']
            print(f"\n{section}:")
            for path, size in sorted(by_path.items(), key=lambda x: -x[1]):
                print(f"  {size:>6,}  {size:#08x}  {path}")


# ---------------------------------------------------------------------------
# sym mode
# ---------------------------------------------------------------------------

def _print_sym_matches(pattern: str, entries: list[dict]) -> None:
    matches = [
        e for e in entries
        if pattern in (e.get('subsection') or '')
        or any(pattern in n for n in e['names'])
    ]

    print(f"{pattern}  ({len(matches)} match{'es' if len(matches) != 1 else ''})")

    if not matches:
        print("  (no matches)")
        return

    for e in matches:
        section = e['section']
        size = e['size']
        path = e['path'] or ''

        print(f"  {path} ({section}) {size:,} bytes")
    print()


def cmd_sym(args: argparse.Namespace) -> None:
    entries = parse_entries(args.map_file)
    for pattern in args.symbol:
        _print_sym_matches(pattern, entries)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description='Analyze a GNU linker map file.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__)
    sub = parser.add_subparsers(dest='cmd', required=True)

    p_lib = sub.add_parser('lib', help='Sum section contributions from a library')
    p_lib.add_argument('map_file', help='Path to the .map file')
    p_lib.add_argument('library', help='Substring matched against object file paths')
    p_lib.add_argument('-d', '--details', action='store_true',
                       help='Print per-object breakdown')

    p_sym = sub.add_parser('sym', help='Find a symbol and print its size')
    p_sym.add_argument('map_file', help='Path to the .map file')
    p_sym.add_argument('symbol', nargs='+', help='Substring(s) matched against symbol names')

    args = parser.parse_args()

    if not Path(args.map_file).exists():
        print(f"Error: '{args.map_file}' not found", file=sys.stderr)
        sys.exit(1)

    if args.cmd == 'lib':
        cmd_lib(args)
    else:
        cmd_sym(args)


if __name__ == '__main__':
    main()
