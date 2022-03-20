#!/usr/bin/env bash

set -xe

which convco
which cargo-set-version

NEXT_VERSION=`convco version --bump HEAD`

cargo set-version $NEXT_VERSION
git add Cargo.toml
git commit -m "chore: bump version"
git tag v$NEXT_VERSION

convco changelog > CHANGELOG.md
git add CHANGELOG.md
git commit -m "chore: changelog"
