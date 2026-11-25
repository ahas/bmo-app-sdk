#!/bin/sh
# Builds bmo-app-declare through the testbed's Cargo verb, in this checkout's own build
# directory, and prints the path of the executable it built. Heavy builds go through the
# machine's queue, so this waits its turn and runs whole.
set -eu

SDK=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$SDK"

if command -v bmo-testbed >/dev/null 2>&1; then
  CARGO="bmo-testbed cargo"
else
  # A machine without the testbed installed yet builds the way any Cargo project does.
  CARGO="cargo"
fi

BIN=$($CARGO build --bin bmo-app-declare --message-format=json 2>/dev/null |
  jq -r 'select(.reason == "compiler-artifact" and .target.name == "bmo-app-declare" and
                 .executable != null) | .executable' |
  tail -n 1)

if [ -z "$BIN" ] || [ ! -x "$BIN" ]; then
  echo "build-generator: the build named no executable" >&2
  exit 1
fi
printf '%s\n' "$BIN"
