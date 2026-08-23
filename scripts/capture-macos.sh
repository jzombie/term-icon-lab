#!/usr/bin/env bash
# macOS capture orchestrator: osascript drives Terminal.app (no TCC mutation —
# runner images pre-grant ScreenCapture to the invoking shell); each page is
# captured with `screencapture -x -l<windowid>`.
#
# Usage: ./scripts/capture-macos.sh [OUT_DIR]   (default: target/matrix)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/page-loop.sh
source "$SCRIPT_DIR/lib/page-loop.sh"

OUT_DIR="${1:-target/matrix}"
HOST="terminal-app"
PLATFORM="macos/$HOST"
HARNESS_BIN="./target/debug/matrix-harness"

mkdir -p "$OUT_DIR/pages" "$OUT_DIR/artifacts"
SYNC_DIR="$(mktemp -d)"

# Launch Terminal.app running the harness. The wrapper publishes the harness
# PID; window size is set for a >=30x16 viewport.
osascript <<APPLESCRIPT
tell application "Terminal"
    activate
    set win to do script "cd '$PWD' && echo \$\$ > '$SYNC_DIR/pid' && exec '$HARNESS_BIN' --sync-dir '$SYNC_DIR' --out-dir '$OUT_DIR/artifacts' --platform '$PLATFORM' --host '$HOST'"
    delay 1
    set background color of tab 1 of front window to {0, 0, 0}
    set normal text color of tab 1 of front window to {49152, 49152, 49152}
    set winid to id of front window
end tell
do shell script "echo " & winid & " > '$SYNC_DIR/winid'"
APPLESCRIPT

for _ in $(seq 1 100); do
    [ -f "$SYNC_DIR/pid" ] && break
    sleep 0.1
done
HARNESS_PID="$(cat "$SYNC_DIR/pid")"
WIN_ID="$(cat "$SYNC_DIR/winid")"

capture_page() {
    local page="$1"
    local shot="${OUT_DIR}/pages/shot_page_${page}.png"
    screencapture -x -l "$WIN_ID" "$shot"

    # Validation gate: reject desktop-only / blank frames, retry once.
    if ! ./target/debug/pixel-assert --validate-only "$shot"; then
        sleep 1
        screencapture -x -l "$WIN_ID" "$shot"
        if ! ./target/debug/pixel-assert --validate-only "$shot"; then
            echo "ERROR: capture validation failed twice for page ${page}." >&2
            kill "$HARNESS_PID" 2>/dev/null || true
            exit 2
        fi
    fi
}

PAGE=0
while kill -0 "$HARNESS_PID" 2>/dev/null; do
    wait_ready_and_acknowledge "$SYNC_DIR" "$HARNESS_PID" "$PAGE" capture_page || {
        STATUS=$?; kill "$HARNESS_PID" 2>/dev/null || true; exit "$STATUS";
    }
    PAGE=$((PAGE + 1))
done
wait "$HARNESS_PID" 2>/dev/null || true

PNG_ARGS=()
for f in "$OUT_DIR"/pages/shot_page_*.png; do PNG_ARGS+=(--png "$f"); done
./target/debug/pixel-assert \
    "${PNG_ARGS[@]}" \
    --sidecar "$OUT_DIR/artifacts/sidecar.json" \
    --pass1 "$OUT_DIR/artifacts/pass1.json" \
    --out "$OUT_DIR/verdicts.json"

rm -rf "$SYNC_DIR"
