#!/bin/sh
# Generated from bmo-app.toml by bmo-app-sdk's bmo-app-declare.
# The web app's check: its locked packages installed with the Chromium build the locked Playwright pins (a machine's
# shared browser cache may hold other builds; nothing is fetched where it is there), its types checked, the app built
# for the browser tests the testbed runs after it, and its unit tests run, one file to a process.
# Bun's own folder and /usr/local/bin, where the Node that nuxi runs on stands, go before the path, since a
# check runs with the path of whoever asked for it.
set -eu
export PATH="$HOME/.bun/bin:/usr/local/bin:$PATH"
cd "$(dirname "$0")/.."
bun install --frozen-lockfile
bun node_modules/playwright/cli.js install --only-shell chromium
bun run typecheck
bun run build
for file in tests/*.test.ts; do
  bun test "$file"
done
