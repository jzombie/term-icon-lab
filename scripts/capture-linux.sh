#!/usr/bin/env bash
# Linux capture orchestrator: Xvfb + xterm (FreeType/DejaVu) → ImageMagick
# `import -window root` per page. Mirrors the ubuntu-latest CI job.
#
# Usage: ./scripts/capture-linux.sh [OUT_DIR]   (default: target/matrix)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/page-loop.sh
source "$SCRIPT_DIR/lib/page-loop.sh"

OUT_DIR="${1:-target/matrix}"
HOST="${TERM_ICON_HOST:-xterm}"
PLATFORM="linux/$HOST"

command -v xvfb-run >/dev/null || { echo "xvfb-run missing"; exit 2; }
command -v xterm >/dev/null || { echo "xterm missing"; exit 2; }
command -v import >/dev/null || { echo "ImageMagick import missing"; exit 2; }

mkdir -p "$OUT_DIR/pages" "$OUT_DIR/artifacts"
SYNC_DIR="$(mktemp -d)"

# Xvfb display: fixed geometry, 24-bit color. Detect an existing server via
# its Unix socket — no extra packages needed.
export DISPLAY="${DISPLAY:-:99}"
DISP_NUM="${DISPLAY#:}"
DISP_NUM="${DISP_NUM%%.*}"
if [ ! -S "/tmp/.X11-unix/X${DISP_NUM}" ]; then
    Xvfb "$DISPLAY" -screen 0 1920x1080x24 &
    XVFB_PID=$!
    trap 'kill ${XVFB_PID:-} 2>/dev/null || true' EXIT
    for _ in $(seq 1 50); do
        [ -S "/tmp/.X11-unix/X${DISP_NUM}" ] && break
        sleep 0.1
    done
fi

# Launch harness inside xterm with explicit FreeType rendering. The wrapper
# publishes the harness PID, sends stderr diagnostics to a log file, and
# records the harness exit code (the harness is not our child, so `wait`
# cannot retrieve its status); stdout must stay attached to the pty — the
# emulator consumes the render and answers its CPR queries there (the xterm
# display itself is invisible to CI logs).
xterm \
    -fa "DejaVu Sans Mono" -fs 11 \
    -geometry 120x48 \
    -e bash -c 'd="$1"; shift; echo $$ > "$d/pid"; "$@" 2>"$d/harness.log"; echo $? > "$d/rc"' bash \
        "$SYNC_DIR" ./target/debug/matrix-harness \
            --sync-dir "$SYNC_DIR" \
            --out-dir "$OUT_DIR/artifacts" \
            --platform "$PLATFORM" \
            --host "$HOST" &
XTERM_PID=$!

for _ in $(seq 1 100); do
    [ -f "$SYNC_DIR/pid" ] && break
    kill -0 "$XTERM_PID" 2>/dev/null || { echo "xterm died early"; exit 2; }
    sleep 0.1
done
HARNESS_PID="$(cat "$SYNC_DIR/pid")"

capture_page() {
    local page="$1"
    import -window root "${OUT_DIR}/pages/shot_page_${page}.png"
}

# The page count is only known from the catalog; drive the loop until the
# harness exits, capturing every signaled page in order. A liveness failure
# (rc=3) after the final page is the graceful end-of-run, not an error —
# verified below via exit code + artifacts.
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

if [ "$RUN_FAILED" -ne 0 ]; then
    echo "=== harness.log tail ===" >&2
    tail -n 40 "$SYNC_DIR/harness.log" 2>/dev/null || true >&2
    exit "$RUN_FAILED"
fi
if [ "$HARNESS_EXIT" -ne 0 ]; then
    echo "ERROR: matrix-harness exited with code $HARNESS_EXIT." >&2
    tail -n 40 "$SYNC_DIR/harness.log" 2>/dev/null || true >&2
    exit 2
fi
for f in sidecar.json pass1.json; do
    if [ ! -f "$OUT_DIR/artifacts/$f" ]; then
        echo "ERROR: harness finished but $f is missing." >&2
        tail -n 40 "$SYNC_DIR/harness.log" 2>/dev/null || true >&2
        exit 2
    fi
done

echo "captured $((PAGE)) pages; harness exit=$HARNESS_EXIT"

# Pass 2 over all captured pages.
PNG_ARGS=()
for f in "$OUT_DIR"/pages/shot_page_*.png; do PNG_ARGS+=(--png "$f"); done
./target/debug/pixel-assert \
    "${PNG_ARGS[@]}" \
    --sidecar "$OUT_DIR/artifacts/sidecar.json" \
    --pass1 "$OUT_DIR/artifacts/pass1.json" \
    --out "$OUT_DIR/verdicts.json"

rm -rf "$SYNC_DIR"
