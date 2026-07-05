#!/usr/bin/env bash
set -euo pipefail

# Builds the Dioxus web bundle and stages it in apps/frontend/dist/web for
# the nginx Docker image. dx is the dioxus-cli binary; the explicit cargo-bin
# default avoids the PATH collision with deno's `dx` alias.
DX_BIN="${DX_BIN:-$HOME/.cargo/bin/dx}"

cd "$(dirname "$0")/../apps/frontend"

# dx doesn't clean its output dir; stale hashed bundles would otherwise
# accumulate and get shipped into the nginx image.
rm -rf target/dx/frontend/release/web/public

"$DX_BIN" build --release

rm -rf dist/web
mkdir -p dist
cp -R target/dx/frontend/release/web/public dist/web

echo "Frontend bundle staged in apps/frontend/dist/web"
