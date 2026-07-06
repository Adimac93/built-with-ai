variable "project_id" {
  description = "GCP project ID."
  type        = string
  default     = "built-with-ai-gdg-wroclaw"
}

variable "github_repo" {
  description = "GitHub repository in 'owner/repo' format."
  type        = string
  default     = "Adimac93/built-with-ai"
}

variable "sa_name" {
  description = "Service account name (account_id)."
  type        = string
  default     = "github-actions-sa"
}

variable "pool_name" {
  description = "Workload Identity Pool ID."
  type        = string
  default     = "github-actions-pool"
}

variable "provider_name" {
  description = "Workload Identity Pool Provider ID."
  type        = string
  default     = "github-provider"
}
