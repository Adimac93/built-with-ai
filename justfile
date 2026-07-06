# Heureka — task runner for the three Rust apps (agent, api, frontend).
# `just --list` shows everything; `just ci` mirrors the CI pipeline.

# dioxus-cli; explicit cargo-bin default because deno ships a conflicting `dx` alias
dx := env_var_or_default("DX_BIN", "$HOME/.cargo/bin/dx")

default:
    @just --list

# ── aggregate ────────────────────────────────────────────────────────────────

build: build-agent build-api build-frontend
test: test-agent test-api test-frontend
lint: lint-agent lint-api lint-frontend
fmt:
    cargo fmt --manifest-path apps/agent/Cargo.toml
    cargo fmt --manifest-path apps/api/Cargo.toml
    cargo fmt --manifest-path apps/frontend/Cargo.toml

# what CI runs
ci: lint test build

# run all three apps (agent :8000, api :3000, frontend :8080); Ctrl-C stops all
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    trap 'kill 0' EXIT
    just serve-agent &
    just serve-api &
    just serve-frontend &
    wait

# ── agent (apps/agent → Cloud Run us-east1) ─────────────────────────────────

build-agent:
    cargo build --release --manifest-path apps/agent/Cargo.toml

test-agent:
    cargo test --manifest-path apps/agent/Cargo.toml

lint-agent:
    cargo clippy --all-targets --manifest-path apps/agent/Cargo.toml -- -D warnings

# local Vertex auth via gcloud user token (expires ~1h)
serve-agent:
    cd apps/agent && GOOGLE_CLOUD_ACCESS_TOKEN="$(gcloud auth print-access-token)" cargo run --release

deploy-agent:
    bash tools/deploy-agent.sh

# ── api (apps/api → Cloud Run us-central1) ──────────────────────────────────

build-api:
    cargo build --release --manifest-path apps/api/Cargo.toml

test-api:
    cargo test --manifest-path apps/api/Cargo.toml

lint-api:
    cargo clippy --all-targets --manifest-path apps/api/Cargo.toml -- -D warnings

# AGENT_URL defaults to http://localhost:8000 inside the binary
serve-api:
    cargo run --release --manifest-path apps/api/Cargo.toml

# resolves AGENT_URL from the deployed agent service — deploy agent first
deploy-api: deploy-agent
    bash tools/deploy-api.sh

# ── frontend (apps/frontend, Dioxus/WASM → Cloud Run us-central1) ───────────

build-frontend:
    bash tools/build-frontend.sh

test-frontend:
    cargo test --manifest-path apps/frontend/Cargo.toml

lint-frontend:
    cd apps/frontend && cargo clippy --target wasm32-unknown-unknown -- -D warnings

serve-frontend:
    cd apps/frontend && {{dx}} serve --port 8080

deploy-frontend: build-frontend
    bash tools/deploy-frontend.sh

# full deploy in dependency order
deploy: deploy-api deploy-frontend
