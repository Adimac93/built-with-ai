variable "project_id" {
  description = "Google Cloud project ID."
  type        = string
  default     = "built-with-ai-gdg-wroclaw"
}

variable "region" {
  description = "Default GCP region for resource deployment."
  type        = string
  default     = "europe-central2"
}

# ── WIF variables ─────────────────────────────────────────────────────────────

variable "github_repo" {
  description = "GitHub repository in 'owner/repo' format used for Workload Identity Federation."
  type        = string
  default     = "Adimac93/built-with-ai"
}

variable "wif_sa_name" {
  description = "Service account name (account_id) used by GitHub Actions."
  type        = string
  default     = "github-actions-sa"
}

variable "wif_pool_name" {
  description = "Workload Identity Pool ID."
  type        = string
  default     = "github-actions-pool"
}

variable "wif_provider_name" {
  description = "Workload Identity Pool Provider ID."
  type        = string
  default     = "github-provider"
}

# ── Agent variables ───────────────────────────────────────────────────────────

variable "agent_project_name" {
  description = "Project name used as a base for agent resource naming."
  type        = string
  default     = "agent"
}
