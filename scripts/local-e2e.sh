#!/usr/bin/env bash
# Local end-to-end replica of the ubuntu CI job (Linux dev machines).
# Requires: xvfb, xterm, imagemagick, fonts-dejavu-core.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> building tools"
cargo build -p matrix-harness -p pixel-assert

echo "==> harness self-check (no TTY)"
cargo run -q -p matrix-harness -- --self-check --out-dir /tmp/opencode/ti-self-check

echo "==> capture + assert"
./scripts/capture-linux.sh target/matrix

echo "==> done: target/matrix/verdicts.json"
