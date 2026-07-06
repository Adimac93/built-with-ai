# Heureka — task runner for the Rust workspace (agent, api, frontend).
# `just --list` shows everything; `just ci` mirrors the CI pipeline.

# dioxus-cli; explicit cargo-bin default because deno ships a conflicting `dx` alias
dx := env_var_or_default("DX_BIN", "$HOME/.cargo/bin/dx")

# Google Cloud deploy targets
project_id := env_var_or_default("PROJECT_ID", "built-with-ai-gdg-wroclaw")
region := env_var_or_default("REGION", "us-central1")
agent_region := env_var_or_default("AGENT_REGION", "us-east1")

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
    #!/usr/bin/env bash
    set -euo pipefail
    image="gcr.io/{{project_id}}/agent"
    echo "Deploying agent to Cloud Run {{project_id}}/{{agent_region}}"
    docker build --platform linux/amd64 -t "$image" -f apps/agent/Dockerfile .
    docker push "$image"
    gcloud run deploy agent \
      --image "$image" \
      --platform managed \
      --region "{{agent_region}}" \
      --allow-unauthenticated \
      --project "{{project_id}}" \
      --update-env-vars "GOOGLE_CLOUD_PROJECT={{project_id}},GOOGLE_CLOUD_LOCATION=global"

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
    #!/usr/bin/env bash
    set -euo pipefail
    image="gcr.io/{{project_id}}/api"
    echo "Resolving agent Cloud Run URL in {{project_id}}/{{agent_region}}..."
    agent_url="$(
      gcloud run services describe agent \
        --platform managed \
        --region "{{agent_region}}" \
        --project "{{project_id}}" \
        --format='value(status.url)'
    )"
    if [[ -z "$agent_url" ]]; then
      echo "Could not resolve AGENT_URL for Cloud Run service 'agent'." >&2
      exit 1
    fi
    echo "Deploying api with AGENT_URL=$agent_url"
    docker build --no-cache --platform linux/amd64 -t "$image" -f apps/api/Dockerfile .
    docker push "$image"
    gcloud run deploy api \
      --image "$image" \
      --platform managed \
      --region "{{region}}" \
      --allow-unauthenticated \
      --project "{{project_id}}" \
      --update-env-vars "AGENT_URL=$agent_url"

# ── frontend (apps/frontend, Dioxus/WASM → Cloud Run us-central1) ───────────

# dx writes into the workspace target dir; stale hashed bundles are purged so
# they don't accumulate into the nginx image. Staged output: apps/frontend/dist/web
build-frontend:
    #!/usr/bin/env bash
    set -euo pipefail
    dx_out="{{justfile_directory()}}/target/dx/frontend/release/web/public"
    cd "{{justfile_directory()}}/apps/frontend"
    rm -rf "$dx_out"
    "{{dx}}" build --release
    rm -rf dist/web
    mkdir -p dist
    cp -R "$dx_out" dist/web
    echo "Frontend bundle staged in apps/frontend/dist/web"

test-frontend:
    cargo test -p frontend

lint-frontend:
    cargo clippy --target wasm32-unknown-unknown -p frontend -- -D warnings

serve-frontend:
    cd apps/frontend && {{dx}} serve --port 8080

deploy-frontend: build-frontend
    #!/usr/bin/env bash
    set -euo pipefail
    image="gcr.io/{{project_id}}/frontend"
    echo "Deploying frontend to Cloud Run {{project_id}}/{{region}}"
    docker build --platform linux/amd64 -t "$image" -f apps/frontend/Dockerfile .
    docker push "$image"
    gcloud run deploy frontend \
      --image "$image" \
      --platform managed \
      --region "{{region}}" \
      --allow-unauthenticated \
      --project "{{project_id}}"

# full deploy in dependency order
deploy: deploy-api deploy-frontend
