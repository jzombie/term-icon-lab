#!/usr/bin/env bash
# Shared page-loop helpers for capture orchestrators (bash platforms).
#
# Contract with matrix-harness:
#   * harness is launched with --sync-dir "$SYNC_DIR"
#   * for each page N: harness writes "$SYNC_DIR"/term_icon_<pid>_page_N.ready
#     after the emulator consumed the page; orchestrator captures
#     shot_page_N.png then creates the .ack file.
#   * every poll iteration checks harness PID liveness so a fail-fast exit
#     aborts the loop immediately instead of burning the full deadline.
set -euo pipefail

HARNESS_ACK_TIMEOUT_SECS="${HARNESS_ACK_TIMEOUT_SECS:-60}"

wait_ready_and_acknowledge() {
    # $1 = sync dir, $2 = harness pid, $3 = page number, $4 = capture command
    # Return codes: 0 = acked; 2 = timed out waiting for .ready;
    #               3 = harness process died (caller decides if that is the
    #                   graceful end-of-run or a mid-run failure).
    local sync_dir="$1" harness_pid="$2" page="$3" capture_cmd="$4"
    local ready ack
    ready="$(find "$sync_dir" -name "term_icon_*_page_${page}.ready" 2>/dev/null | head -n1 || true)"
    ack=""

    local deadline=$((SECONDS + HARNESS_ACK_TIMEOUT_SECS))
    while [ -z "$ready" ]; do
        if ! kill -0 "$harness_pid" 2>/dev/null; then
            echo "NOTE: matrix-harness pid $harness_pid exited before signaling page ${page}." >&2
            return 3
        fi
        if (( SECONDS >= deadline )); then
            echo "ERROR: timed out waiting for page ${page} .ready signal." >&2
            return 2
        fi
        sleep 0.05
        ready="$(find "$sync_dir" -name "term_icon_*_page_${page}.ready" 2>/dev/null | head -n1 || true)"
    done

    # Frame consumed by the emulator — capture it.
    "$capture_cmd" "$page"

    ack="${ready%.ready}.ack"
    : > "$ack"

    # Wait for the harness to consume the ack, keeping liveness checks on.
    deadline=$((SECONDS + HARNESS_ACK_TIMEOUT_SECS))
    while [ -f "$ack" ]; do
        if ! kill -0 "$harness_pid" 2>/dev/null; then break; fi   # finished or died; outer loop handles it
        if (( SECONDS >= deadline )); then
            echo "ERROR: harness never consumed ack for page ${page}." >&2
            return 2
        fi
        sleep 0.05
    done
}

run_page_loop() {
    # $1 = pages expected, $2 = sync dir, $3 = harness pid, $4 = capture command
    local pages="$1" sync_dir="$2" harness_pid="$3" capture_cmd="$4"
    local p
    for p in $(seq 0 $((pages - 1))); do
        wait_ready_and_acknowledge "$sync_dir" "$harness_pid" "$p" "$capture_cmd" || return $?
    done
}
