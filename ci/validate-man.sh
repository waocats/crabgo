#!/bin/bash
# This script validates that there aren't any changes to the man pages.

set -e

crabgo_man="src/doc"
mdman_man="crates/mdman/doc"

changes=$(git status --porcelain -- $crabgo_man $mdman_man)
if [ -n "$changes" ]
then
    echo "git directory must be clean before running this script."
    exit 1
fi

crabgo build-man

changes=$(git status --porcelain -- $crabgo_man $mdman_man)
if [ -n "$changes" ]
then
    echo "Detected changes of man pages:"
    echo "$changes"
    echo
    echo 'Please run `crabgo build-man` to rebuild the man pages'
    echo "and commit the changes."
    exit 1
fi
