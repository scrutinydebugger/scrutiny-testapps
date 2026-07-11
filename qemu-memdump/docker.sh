#!/bin/bash
set -euoE pipefail
trap ">&2 echo FAILED;" ERR

cd "$(dirname $0)"
WORKDIR="$(pwd)"
PLATFORM=$1
shift

RED='\033[0;31m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'; GREEN='\033[0;32m';
fatal() { >&2 echo -e "$RED[Fatal]$NC $1"; exit ${2:-1}; }

[ -z ${PLATFORM-""} ] && fatal "Missing platform"

docker run                          \
    --volume $WORKDIR:$WORKDIR      \
    -w $WORKDIR                     \
    -u $(id -u):$(id -g)            \
    --env PLATFORM="$PLATFORM"      \
    ${PLATFORM}                     \
    bash -c 'source /tmp/venv/bin/activate; exec "$@"' -- "$@"