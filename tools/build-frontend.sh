#!/usr/bin/env bash
set -euo pipefail

# Builds the Dioxus web bundle and stages it in apps/frontend/dist/web for
# the nginx Docker image. dx is the dioxus-cli binary; the explicit cargo-bin
# default avoids the PATH collision with deno's `dx` alias.
DX_BIN="${DX_BIN:-$HOME/.cargo/bin/dx}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# dx writes into the cargo workspace target dir.
DX_OUT="$ROOT/target/dx/frontend/release/web/public"

cd "$ROOT/apps/frontend"

# dx doesn't clean its output dir; stale hashed bundles would otherwise
# accumulate and get shipped into the nginx image.
rm -rf "$DX_OUT"

"$DX_BIN" build --release

rm -rf dist/web
mkdir -p dist
cp -R "$DX_OUT" dist/web

echo "Frontend bundle staged in apps/frontend/dist/web"
