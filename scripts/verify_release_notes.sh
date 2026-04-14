#!/usr/bin/env bash

set -euo pipefail

STACKABLECTL_VERSION=$(grep 'version' "./rust/stackablectl/Cargo.toml" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
[ -n "$STACKABLECTL_VERSION" ] || (
    echo "CRATE_VERSION for is empty." >&2
    echo "Please check ./rust/stackablectl/Cargo.toml" >&2
    exit 1
)

RELEASE_NOTES_FILE="./docs/modules/stackablectl/partials/release-notes/release-${STACKABLECTL_VERSION}.adoc"

if [ ! -f "$RELEASE_NOTES_FILE" ]; then
    echo "$RELEASE_NOTES_FILE does not exist, please create it and add the appropriate content" >&2
    exit 1
fi
