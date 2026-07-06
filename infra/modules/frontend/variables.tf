variable "project_id" {
  type        = string
  description = "Google Cloud Project ID for resource deployment."
}

variable "region" {
  type        = string
  description = "Google Cloud region for resource deployment."
  default     = "europe-central2"
}

variable "agent_url" {
  type        = string
  description = "URL of the deployed agent Cloud Run service, injected as AGENT_URL into the frontend container."
}

variable "allow_unauthenticated_invocations" {
  type        = bool
  description = "Grant allUsers the run.invoker role so the frontend is publicly accessible."
  default     = true
}
