#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd $SCRIPT_DIR

APP=app/build/footprint-app.elf
MAPFILE=app/build/footprint-app.map
BUILD_DIR="app/build"

echo "========= Stack Usage ========="
echo "==============================="

python3 "./scrutiny-devtools/stackusage.py" "$BUILD_DIR" idle loop1

echo "==== libscrutiny-embedded Footprint ===="
echo "========================================"
python3 ./footprint.py lib  "$MAPFILE" libscrutiny-embedded -d

echo
echo "============= Symbols ==============="
python3 ./footprint.py sym "$MAPFILE" main_handler loop_handler1
