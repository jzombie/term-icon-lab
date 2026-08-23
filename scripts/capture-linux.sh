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
# publishes the harness PID for liveness polling.
xterm \
    -fa "DejaVu Sans Mono" -fs 11 \
    -geometry 120x48 \
    -e bash -c 'echo $$ > "$1/pid"; shift 2>/dev/null || true; exec "$@"' bash \
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
# harness exits, capturing every signaled page in order.
PAGE=0
SECONDS=0
while kill -0 "$HARNESS_PID" 2>/dev/null; do
    wait_ready_and_acknowledge "$SYNC_DIR" "$HARNESS_PID" "$PAGE" capture_page || {
        STATUS=$?; kill "$HARNESS_PID" 2>/dev/null || true; exit "$STATUS";
    }
    PAGE=$((PAGE + 1))
done
wait "$HARNESS_PID" 2>/dev/null || true

echo "captured $((PAGE)) pages"

# Pass 2 over all captured pages.
PNG_ARGS=()
for f in "$OUT_DIR"/pages/shot_page_*.png; do PNG_ARGS+=(--png "$f"); done
./target/debug/pixel-assert \
    "${PNG_ARGS[@]}" \
    --sidecar "$OUT_DIR/artifacts/sidecar.json" \
    --pass1 "$OUT_DIR/artifacts/pass1.json" \
    --out "$OUT_DIR/verdicts.json"

rm -rf "$SYNC_DIR"
