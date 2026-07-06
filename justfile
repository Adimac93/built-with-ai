# Heureka — task runner for the Rust workspace (agent, fullstack frontend).
# `just --list` shows everything; `just ci` mirrors the CI pipeline.

# Google Cloud deploy targets
project_id := env_var_or_default("PROJECT_ID", "built-with-ai-gdg-wroclaw")
# must match .github/workflows/deploy.yml and infra/ Terraform
region := env_var_or_default("REGION", "europe-central2")
agent_region := env_var_or_default("AGENT_REGION", "europe-central2")

default:
    @just --list

# ── aggregate ────────────────────────────────────────────────────────────────

build: build-agent build-frontend

# workspace default features + the frontend server side (cfg'd out of --workspace)
test:
    cargo test --workspace
    cargo test -p frontend --no-default-features --features server

lint: lint-agent lint-frontend

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

# supply-chain: advisories, licenses, bans, sources (deny.toml)
deny:
    cargo deny check

# what CI runs
ci: fmt-check lint deny test build

# run agent (:8000) + fullstack frontend (:8080); Ctrl-C stops both
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    trap 'kill 0' EXIT
    just serve-agent &
    just serve-frontend &
    wait

# ── agent (apps/agent → Cloud Run, region = AGENT_REGION) ───────────────────

build-agent:
    cargo build --release -p agent

test-agent:
    cargo test -p agent

lint-agent:
    cargo clippy --all-targets -p agent -- -D warnings

# local Vertex auth via gcloud user token (expires ~1h);
# cd needed: dotenvy finds apps/agent/.env from cwd/ancestors only
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

# ── frontend (apps/frontend, Dioxus fullstack → Cloud Run, region = REGION) ─

# Build and stage the Dioxus fullstack bundle in apps/frontend/dist/web.
build-frontend:
    #!/usr/bin/env bash
    set -euo pipefail
    dx_out="{{justfile_directory()}}/target/dx/frontend/release/web"
    dist="{{justfile_directory()}}/apps/frontend/dist"
    rm -rf "$dx_out"
    dx build -p frontend --release --fullstack --force-sequential
    rm -rf "$dist/web"
    mkdir -p "$dist"
    cp -R "$dx_out" "$dist/web"
    echo "Frontend bundle staged in apps/frontend/dist/web"

test-frontend:
    cargo test -p frontend
    cargo test -p frontend --no-default-features --features server

lint-frontend:
    cargo clippy --target wasm32-unknown-unknown -p frontend -- -D warnings
    cargo clippy -p frontend --no-default-features --features server -- -D warnings

serve-frontend:
    dx serve -p frontend --fullstack --port 8080

deploy-frontend: deploy-agent
    #!/usr/bin/env bash
    set -euo pipefail
    image="gcr.io/{{project_id}}/frontend"
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
    echo "Deploying frontend to Cloud Run {{project_id}}/{{region}} with AGENT_URL=$agent_url"
    docker build --platform linux/amd64 -t "$image" -f apps/frontend/Dockerfile .
    docker push "$image"
    gcloud run deploy frontend \
      --image "$image" \
      --platform managed \
      --region "{{region}}" \
      --allow-unauthenticated \
      --project "{{project_id}}" \
      --update-env-vars "AGENT_URL=$agent_url"

# full deploy in dependency order
deploy: deploy-frontend
