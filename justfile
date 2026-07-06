# Heureka — task runner for the Rust workspace (agent, api, frontend).
# `just --list` shows everything; `just ci` mirrors the CI pipeline.

# dioxus-cli; explicit cargo-bin default because deno ships a conflicting `dx` alias
dx := env_var_or_default("DX_BIN", "$HOME/.cargo/bin/dx")

default:
    @just --list

# ── aggregate ────────────────────────────────────────────────────────────────

build: build-agent build-api build-frontend

test:
    cargo test --workspace

lint: lint-agent lint-api lint-frontend

fmt:
    cargo fmt --all

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
    cargo build --release -p agent

test-agent:
    cargo test -p agent

lint-agent:
    cargo clippy --all-targets -p agent -- -D warnings

# local Vertex auth via gcloud user token (expires ~1h)
serve-agent:
    cd apps/agent && GOOGLE_CLOUD_ACCESS_TOKEN="$(gcloud auth print-access-token)" cargo run --release -p agent

deploy-agent:
    bash tools/deploy-agent.sh

# ── api (apps/api → Cloud Run us-central1) ──────────────────────────────────

build-api:
    cargo build --release -p api

test-api:
    cargo test -p api

lint-api:
    cargo clippy --all-targets -p api -- -D warnings

# AGENT_URL defaults to http://localhost:8000 inside the binary
serve-api:
    cargo run --release -p api

# resolves AGENT_URL from the deployed agent service — deploy agent first
deploy-api: deploy-agent
    bash tools/deploy-api.sh

# ── frontend (apps/frontend, Dioxus/WASM → Cloud Run us-central1) ───────────

build-frontend:
    bash tools/build-frontend.sh

test-frontend:
    cargo test -p frontend

lint-frontend:
    cargo clippy --target wasm32-unknown-unknown -p frontend -- -D warnings

serve-frontend:
    cd apps/frontend && {{dx}} serve --port 8080

deploy-frontend: build-frontend
    bash tools/deploy-frontend.sh

# full deploy in dependency order
deploy: deploy-api deploy-frontend
