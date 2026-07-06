# Cloud Run observability: tracing-stackdriver behind a K_SERVICE switch

Date: 2026-07-06
Status: accepted

## Context

The agent logged through `tracing_subscriber::fmt()` — plain text lines. Cloud
Logging ingests those as unstructured `textPayload` with default severity, so
`tracing::error!` was indistinguishable from `info!` when filtering incidents.
The `/feedback` route already worked around this by hand-printing one JSON line
whose fields (`severity`, `log_type`, `service_name`) the BigQuery feedback
sink filter matches.

Plain `fmt().json()` would not fix severity: Cloud Logging maps the `severity`
field, not tracing's `level`.

## Decision

- Add `tracing-stackdriver` (0.10) and emit its Cloud Logging-compatible JSON
  (severity, sourceLocation, flattened fields) when `K_SERVICE` is set — the
  env var Cloud Run injects into every revision.
- Keep human-readable `fmt` output locally.
- Filter via `EnvFilter` (`RUST_LOG`, default `info`) in both modes.
- The `/feedback` `println!` stays untouched: its exact jsonPayload shape is a
  frozen contract with the Terraform-managed BigQuery sink
  (`deployment/terraform/single-project/telemetry.tf`).

## Consequences

- Errors and warnings filter correctly in Cloud Logging; log fields become
  queryable `jsonPayload` keys.
- One extra dependency in the agent only; the frontend server keeps dioxus's
  default logger (its logging is incidental — request handling lives in the
  agent).
- If `tracing-stackdriver` stalls upstream, the fallback is a custom
  `FormatEvent` impl mapping `level` → `severity`; the switch point is a
  single `init_tracing` function in `apps/agent/src/main.rs`.
