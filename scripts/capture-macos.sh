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
# PID; explicit window bounds guarantee the >=30x16 viewport the harness
# enforces. Colors are pinned by the harness itself (SGR), so no profile
# mutation happens here.
osascript <<APPLESCRIPT
tell application "Terminal"
    activate
    set win to do script "cd '$PWD' && echo \$\$ > '$SYNC_DIR/pid' && exec '$HARNESS_BIN' --sync-dir '$SYNC_DIR' --out-dir '$OUT_DIR/artifacts' --platform '$PLATFORM' --host '$HOST'"
    delay 1
    set bounds of front window to {0, 0, 980, 760}
end tell
APPLESCRIPT

for _ in $(seq 1 100); do
    [ -f "$SYNC_DIR/pid" ] && break
    sleep 0.1
done
HARNESS_PID="$(cat "$SYNC_DIR/pid")"

# Resolve the real CGWindowID: Terminal.app's AppleScript `id of window`
# returns a session GUID string, which screencapture -l cannot consume. A tiny
# Swift tool queries CGWindowListCopyWindowInfo for the on-screen, layer-0
# window owned by Terminal and prints its numeric kCGWindowNumber.
cat > "$SYNC_DIR/winid.swift" <<'SWIFT'
import CoreGraphics
import Foundation
let opts: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let list = CGWindowListCopyWindowInfo(opts, kCGNullWindowID) as? [[String: Any]] ?? []
for w in list {
    let owner = w[kCGWindowOwnerName as String] as? String ?? ""
    let layer = w[kCGWindowLayer as String] as? Int ?? -1
    if owner == "Terminal", layer == 0,
       let n = w[kCGWindowNumber as String] as? Int {
        print(n)
        exit(0)
    }
}
FileHandle.standardError.write(Data("no Terminal window found\n".utf8))
exit(1)
SWIFT
swiftc -O "$SYNC_DIR/winid.swift" -o "$SYNC_DIR/winidtool"

WIN_ID=""
for _ in $(seq 1 50); do
    if "$SYNC_DIR/winidtool" > "$SYNC_DIR/winid" 2>/dev/null; then
        WIN_ID="$(tr -dc '[:digit:]' < "$SYNC_DIR/winid")"
        [ -n "$WIN_ID" ] && break
    fi
    kill -0 "$HARNESS_PID" 2>/dev/null || break
    sleep 0.2
done
if [ -z "${WIN_ID:-}" ] || ! [[ "$WIN_ID" =~ ^[0-9]+$ ]]; then
    echo "ERROR: could not resolve numeric CGWindowID for Terminal.app." >&2
    echo "Diagnostics — on-screen windows:" >&2
    "$SYNC_DIR/winidtool" 2>&1 || true
    kill "$HARNESS_PID" 2>/dev/null || true
    exit 2
fi

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
