# ── APIs ─────────────────────────────────────────────────────────────────────

resource "google_project_service" "iam_credentials" {
  service            = "iamcredentials.googleapis.com"
  disable_on_destroy = false
}

# ── Service Account ──────────────────────────────────────────────────────────

resource "google_service_account" "github_actions" {
  account_id   = var.sa_name
  display_name = "GitHub Actions Service Account"
  project      = var.project_id
}

# ── IAM roles on the project ─────────────────────────────────────────────────

locals {
  sa_roles = [
    "roles/run.admin",
    "roles/iam.serviceAccountUser",
    "roles/artifactregistry.writer",
    "roles/storage.admin",
  ]
}

resource "google_project_iam_member" "github_actions_roles" {
  for_each = toset(local.sa_roles)

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.github_actions.email}"
}

# ── Workload Identity Pool ───────────────────────────────────────────────────

resource "google_iam_workload_identity_pool" "github" {
  workload_identity_pool_id = var.pool_name
  display_name              = "GitHub Actions Pool"
  project                   = var.project_id

  depends_on = [google_project_service.iam_credentials]
}

# ── OIDC Provider ────────────────────────────────────────────────────────────

resource "google_iam_workload_identity_pool_provider" "github" {
  workload_identity_pool_id          = google_iam_workload_identity_pool.github.workload_identity_pool_id
  workload_identity_pool_provider_id = var.provider_name
  display_name                       = "GitHub Actions Provider"
  project                            = var.project_id

  attribute_mapping = {
    "google.subject"       = "assertion.sub"
    "attribute.actor"      = "assertion.actor"
    "attribute.repository" = "assertion.repository"
  }

  attribute_condition = "assertion.repository == '${var.github_repo}'"

  oidc {
    issuer_uri = "https://token.actions.githubusercontent.com"
  }
}

# ── Bind Service Account to the GitHub repo principal set ────────────────────

resource "google_service_account_iam_member" "github_wif_binding" {
  service_account_id = google_service_account.github_actions.name
  role               = "roles/iam.workloadIdentityUser"

  # principalSet grants any workflow in the repo (any branch/ref)
  member = "principalSet://iam.googleapis.com/${google_iam_workload_identity_pool.github.name}/attribute.repository/${var.github_repo}"
}
