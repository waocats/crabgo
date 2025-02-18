#!/bin/bash
# This script builds with static curl, and verifies that fetching works.

set -ex

if [[ -z "$RUNNER_TEMP" ]]
then
    echo "RUNNER_TEMP must be set"
    exit 1
fi

if [ ! -f Crabgo.toml ]; then
    echo "Must be run from root of project."
    exit 1
fi


# Building openssl on Windows is a pain.
if [[ $(rustc -Vv | grep host:) != *windows* ]]; then
    FEATURES='vendored-openssl,curl-sys/static-curl,curl-sys/force-system-lib-on-osx'
    export LIBZ_SYS_STATIC=1
fi

crabgo build --features "$FEATURES"
export CRABGO_HOME=$RUNNER_TEMP/chome
target/debug/crabgo fetch
rm -rf $CRABGO_HOME
