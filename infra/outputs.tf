output "workload_identity_provider" {
  description = "Full WIF provider resource name — use as WORKLOAD_IDENTITY_PROVIDER in GitHub Actions."
  value       = module.wif.workload_identity_provider
}

output "github_actions_service_account" {
  description = "GitHub Actions SA email — use as SERVICE_ACCOUNT in GitHub Actions."
  value       = module.wif.service_account_email
}

output "agent_service_url" {
  description = "Cloud Run URL for the deployed agent service."
  value       = module.agent.cloud_run_service_url
}

output "agent_service_account_email" {
  description = "Service account email used by the agent Cloud Run service."
  value       = module.agent.app_service_account_email
}

output "agent_logs_bucket" {
  description = "GCS bucket name for agent telemetry logs."
  value       = module.agent.logs_bucket_name
}

output "telemetry_dataset_id" {
  description = "BigQuery dataset ID for agent telemetry."
  value       = module.agent.telemetry_dataset_id
}

output "frontend_url" {
  description = "Public URL of the Dioxus frontend."
  value       = module.frontend.service_url
}

