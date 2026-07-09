#!/bin/bash
set -euoE pipefail
trap ">&2 echo FAILED;" ERR

cd "$(dirname $0)"
WORKDIR="$(pwd)"

docker run                          \
    --volume $WORKDIR:$WORKDIR      \
    -v /tmp:/tmp    \
    -w $WORKDIR                     \
    -u $(id -u):$(id -g)            \
    footprint                      \
    bash -c 'exec "$@"' -- "$@"