#!/usr/bin/env bash
# migrate-state.sh — re-address state entries from the old flat single-project
# layout to the new module.agent.* hierarchy.
#
# Run ONCE from the infra/ directory after `terraform init`:
#
#   cd infra
#   terraform init
#   bash migrate-state.sh
#
# Safe to re-run: terraform state mv is a no-op if the destination already exists.

set -euo pipefail

die() { echo "ERROR: $*" >&2; exit 1; }
[[ -f terraform.tfstate ]] || die "Run this script from the infra/ directory containing terraform.tfstate"

echo "=== Migrating state to module.agent.* ==="

mv_if_exists() {
  local src="$1" dst="$2"
  if terraform state list "$src" 2>/dev/null | grep -qF "$src"; then
    echo "  mv $src → $dst"
    terraform state mv "$src" "$dst"
  else
    echo "  skip (not found): $src"
  fi
}

# ── Simple resources ──────────────────────────────────────────────────────────
mv_if_exists "data.google_project.project"                                         "module.agent.data.google_project.project"
mv_if_exists "google_bigquery_connection.genai_telemetry_connection"               "module.agent.google_bigquery_connection.genai_telemetry_connection"
mv_if_exists "google_bigquery_dataset.telemetry_dataset"                           "module.agent.google_bigquery_dataset.telemetry_dataset"
mv_if_exists "google_bigquery_dataset_iam_member.feedback_logs_bq_writer"         "module.agent.google_bigquery_dataset_iam_member.feedback_logs_bq_writer"
mv_if_exists "google_bigquery_dataset_iam_member.genai_logs_bq_writer"            "module.agent.google_bigquery_dataset_iam_member.genai_logs_bq_writer"
mv_if_exists "google_bigquery_table.completions_external_table"                    "module.agent.google_bigquery_table.completions_external_table"
mv_if_exists "google_bigquery_table.completions_view"                              "module.agent.google_bigquery_table.completions_view"
mv_if_exists "google_bigquery_table.genai_logs_table"                             "module.agent.google_bigquery_table.genai_logs_table"
mv_if_exists "google_cloud_run_v2_service.app"                                     "module.agent.google_cloud_run_v2_service.app"
mv_if_exists "google_logging_project_sink.feedback_logs_to_bq"                    "module.agent.google_logging_project_sink.feedback_logs_to_bq"
mv_if_exists "google_logging_project_sink.genai_logs_to_bq"                       "module.agent.google_logging_project_sink.genai_logs_to_bq"
mv_if_exists "google_project_iam_member.default_compute_sa_storage_object_creator" "module.agent.google_project_iam_member.default_compute_sa_storage_object_creator"
mv_if_exists "google_service_account.app_sa"                                       "module.agent.google_service_account.app_sa"
mv_if_exists "google_storage_bucket.logs_data_bucket"                             "module.agent.google_storage_bucket.logs_data_bucket"
mv_if_exists "google_storage_bucket_iam_member.telemetry_connection_access"       "module.agent.google_storage_bucket_iam_member.telemetry_connection_access"
mv_if_exists "time_sleep.wait_for_bq_connection_sa"                               "module.agent.time_sleep.wait_for_bq_connection_sa"
mv_if_exists "google_project_service_identity.vertex_sa"                          "module.agent.google_project_service_identity.vertex_sa"

# ── Counted resources (google_project_service has count) ──────────────────────
echo "  migrating google_project_service.services[*]..."
for i in $(seq 0 9); do
  mv_if_exists "google_project_service.services[$i]" "module.agent.google_project_service.services[$i]"
done

# ── for_each resources (google_project_iam_member.app_sa_roles) ───────────────
echo "  migrating google_project_iam_member.app_sa_roles[*]..."
while IFS= read -r addr; do
  key="${addr#google_project_iam_member.app_sa_roles}"
  mv_if_exists "$addr" "module.agent.google_project_iam_member.app_sa_roles${key}"
done < <(terraform state list 2>/dev/null | grep '^google_project_iam_member\.app_sa_roles')

echo ""
echo "=== State migration complete. Run 'terraform plan' to verify no destructive changes. ==="
