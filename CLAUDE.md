# Project Context

Heureka — an inventive problem solver (TRIZ + SCAMPER) built entirely in Rust:

- `apps/frontend` — Dioxus 0.7 web app (WASM), UI language is Polish
- `apps/api` — axum gateway (`/solve` proxy, `/speech/transcribe`)
- `apps/agent` — axum service running the TRIZ+SCAMPER pipeline against Gemini (Vertex AI REST)

The three crates are standalone (no cargo workspace). Everything deploys to Google Cloud Run;
the agent's infra is Terraform under `apps/agent/deployment/terraform/`.

# Tasks

Use `just` for all tasks (`just --list` shows everything):

- `just ci` — lint + test + build, exactly what CI runs; run it before claiming work done
- `just dev` — run all three apps locally (agent :8000, api :3000, frontend :8080)
- `just test-api` / `lint-agent` / `build-frontend` etc. — per-app targets
- The frontend builds with `dioxus-cli`; the local `dx` on PATH is deno's alias — scripts
  default to `~/.cargo/bin/dx` (override with `DX_BIN`)

# Constraints

- Clippy is authoritative: `-D warnings`, `--all-targets` (frontend lints for the
  `wasm32-unknown-unknown` target)
- The `/solve` trail JSON shape is a frozen contract between agent, api, and frontend —
  `step2a_lookup.principles[].number` must stay a **string**; the 8 trail keys must not change
- API error bodies mirror the old NestJS shape (`statusCode`/`message`/`error`); the agent
  mirrors FastAPI's `{"detail": ...}`
- `apps/frontend/assets/heureka.css` is compiled output of the old SCSS design tokens —
  edit it directly, there is no SCSS toolchain anymore
- Never change the Gemini model (`gemini-flash-latest`) unless explicitly asked;
  model 404s mean wrong `GOOGLE_CLOUD_LOCATION` (should be `global`), not a wrong model name

# Architecture

## ADR

- Create ADRs in `/docs/adr/{name}.md` for: major dependency changes, architectural pattern
  changes, new integration patterns

## Layering

- Keep deterministic domain logic (TRIZ matrix, argmax choice, trail mapping) in plain
  modules with unit tests, separate from HTTP handlers and LLM calls
- LLMs propose, deterministic code validates and decides — preserve this split in the pipeline

# Documentation

- Document public functions, structs, and modules with rustdoc (`///`), including error and
  panic behavior where relevant
- Focus comments on "why", not "what"

# Collaboration

## Expert Support Level

- Favor elegant, maintainable solutions over verbose code — assume understanding of idioms and design patterns
- Highlight potential performance implications and optimization opportunities
- Frame solutions within broader architectural contexts; suggest design alternatives when appropriate
- Proactively address edge cases, race conditions, and security considerations
- When debugging, provide targeted diagnostic approaches rather than shotgun solutions
- Suggest comprehensive testing strategies including mocking, test organization, and coverage considerations
