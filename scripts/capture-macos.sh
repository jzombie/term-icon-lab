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

# Launch Terminal.app WITHOUT AppleEvents on the critical path: Terminal
# executes .command files natively, and `open -a Terminal` needs no Automation
# consent. (The previous `do script` channel died with AppleEvent timeout
# -1712 on headless runners.) Window bounds are best-effort afterwards; the
# harness's own viewport guard remains the authority.
# Launch contract: publish the harness PID for liveness checks, send stderr
# diagnostics to a log file, record its real exit code (the process is not a
# child of this script, so `wait` cannot retrieve it), and keep stdout on the
# Terminal pty — the emulator consumes the render and answers CPR queries.
cat > "$SYNC_DIR/run_matrix.command" <<WRAP
#!/bin/bash
echo \$\$ > '$SYNC_DIR/pid'
cd '$PWD'
'$HARNESS_BIN' --sync-dir '$SYNC_DIR' --out-dir '$OUT_DIR/artifacts' --platform '$PLATFORM' --host '$HOST' 2>'$SYNC_DIR/harness.log'
echo \$? > '$SYNC_DIR/rc'
WRAP
chmod +x "$SYNC_DIR/run_matrix.command"
open -a Terminal "$SYNC_DIR/run_matrix.command"

for _ in $(seq 1 150); do
    [ -f "$SYNC_DIR/pid" ] && break
    sleep 0.2
done
if [ ! -f "$SYNC_DIR/pid" ]; then
    echo "ERROR: Terminal.app never launched the harness wrapper." >&2
    exit 2
fi
HARNESS_PID="$(cat "$SYNC_DIR/pid")"

# Best-effort resize (needs the Automation channel; failure is non-fatal).
if ! osascript -e 'with timeout of 30 seconds
tell application "Terminal" to set bounds of front window to {0, 0, 980, 760}
end timeout' 2>/dev/null; then
    echo "WARN: window resize skipped (Automation channel unavailable); relying on harness viewport guard."
fi

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
RUN_FAILED=0
while kill -0 "$HARNESS_PID" 2>/dev/null; do
    if ! wait_ready_and_acknowledge "$SYNC_DIR" "$HARNESS_PID" "$PAGE" capture_page; then
        RC=$?
        if [ "$RC" -ne 3 ]; then
            RUN_FAILED=$RC
            kill "$HARNESS_PID" 2>/dev/null || true
        fi
        break
    fi
    PAGE=$((PAGE + 1))
done

set +e
wait "$HARNESS_PID" 2>/dev/null
# `wait` cannot see a non-child process; the real exit code comes from the
# wrapper's rc file (poll briefly — it lands just before the pid goes away).
HARNESS_EXIT=""
for _ in $(seq 1 50); do
    [ -f "$SYNC_DIR/rc" ] && break
    kill -0 "$HARNESS_PID" 2>/dev/null || break
    sleep 0.1
done
[ -z "$HARNESS_EXIT" ] && HARNESS_EXIT="$(cat "$SYNC_DIR/rc" 2>/dev/null || echo unknown)"
set -e

dump_harness_log() {
    echo "=== harness.log tail ===" >&2
    tail -n 40 "$SYNC_DIR/harness.log" 2>/dev/null || true >&2
}

if [ "$RUN_FAILED" -ne 0 ]; then
    dump_harness_log
    exit "$RUN_FAILED"
fi
if [ "$HARNESS_EXIT" -ne 0 ]; then
    echo "ERROR: matrix-harness exited with code $HARNESS_EXIT." >&2
    dump_harness_log
    exit 2
fi
for f in sidecar.json pass1.json; do
    if [ ! -f "$OUT_DIR/artifacts/$f" ]; then
        echo "ERROR: harness finished but $f is missing." >&2
        dump_harness_log
        exit 2
    fi
done

echo "captured $((PAGE)) pages; harness exit=$HARNESS_EXIT"

PNG_ARGS=()
for f in "$OUT_DIR"/pages/shot_page_*.png; do PNG_ARGS+=(--png "$f"); done
./target/debug/pixel-assert \
    "${PNG_ARGS[@]}" \
    --sidecar "$OUT_DIR/artifacts/sidecar.json" \
    --pass1 "$OUT_DIR/artifacts/pass1.json" \
    --out "$OUT_DIR/verdicts.json"

rm -rf "$SYNC_DIR"
