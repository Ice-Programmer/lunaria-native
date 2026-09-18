#!/usr/bin/env bash

set -euo pipefail

PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

if ! command -v watchexec >/dev/null 2>&1; then
    echo "Error: watchexec is not installed."
    echo "Please install it first: brew install watchexec"
    exit 1
fi

exec watchexec \
    --restart \
    --exts rs,toml \
    -- cargo run
