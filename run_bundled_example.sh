#!/usr/bin/env sh
set -e

BOLD_GREEN="\033[1;35m"
RESET="\033[0m"

status() {
    label="$1"
    shift
    printf "%b%12s%b %s\n" "$BOLD_GREEN" "$label" "$RESET" "$*"
}

EXAMPLE="${1:-un_async}"
APP=target/debug/examples/bundle/osx/notify-rust.app

status "Bundling" "$EXAMPLE (example)"
cargo bundle --example "$EXAMPLE" --features preview-macos-un

status "Signing" "$APP (ad-hoc)"
codesign --force --deep --sign - "$APP"

status "Opening" "$APP"
pkill -x "$EXAMPLE" 2>/dev/null || true
open "$APP"

# Give the app a moment to launch, then grab its PID.
sleep 0.5
APP_PID=$(pgrep -x "$EXAMPLE" | head -1)

status "Streaming" "logs for process: $EXAMPLE (PID $APP_PID — stops when app exits)"
# First replay any logs already emitted since the app launched, then stream live.
# Both commands include debug level messages.
PREDICATE="process == \"$EXAMPLE\" AND subsystem == \"notify-rust\""
# Uncomment the line below to also include Apple's internal UserNotifications/XPC logs:
# PREDICATE="process == \"$EXAMPLE\""
log show \
    --predicate "$PREDICATE" \
    --style compact \
    --debug \
    --info \
    --last 2s
log stream \
    --predicate "$PREDICATE" \
    --style compact \
    --level debug &
LOG_PID=$!

# Wait for the app to exit, then stop the log stream.
while kill -0 "$APP_PID" 2>/dev/null; do
    sleep 0.5
done
status "Done" "$EXAMPLE exited — stopping log stream"
kill "$LOG_PID" 2>/dev/null || true
