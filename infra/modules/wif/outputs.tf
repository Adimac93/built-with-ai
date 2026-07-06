output "workload_identity_provider" {
  description = "Full resource name of the WIF provider – use this as WORKLOAD_IDENTITY_PROVIDER in your GitHub Actions workflow."
  value       = google_iam_workload_identity_pool_provider.github.name
}

output "service_account_email" {
  description = "Email of the GitHub Actions service account – use this as SERVICE_ACCOUNT in your GitHub Actions workflow."
  value       = google_service_account.github_actions.email
}
