terraform {
  required_version = ">= 1.5"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 7.28"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 7.28"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.7"
    }
    time = {
      source  = "hashicorp/time"
      version = "~> 0.14"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  project               = var.project_id
  region                = var.region
  billing_project       = var.project_id
  user_project_override = true
}

# ── Workload Identity Federation (GitHub Actions) ─────────────────────────────

module "wif" {
  source = "./modules/wif"

  project_id    = var.project_id
  github_repo   = var.github_repo
  sa_name       = var.wif_sa_name
  pool_name     = var.wif_pool_name
  provider_name = var.wif_provider_name
}

# ── Agent infrastructure ──────────────────────────────────────────────────────

module "agent" {
  source = "./modules/agent"

  project_id   = var.project_id
  project_name = var.agent_project_name
  region       = var.region
}

# ── Frontend (Dioxus fullstack) ───────────────────────────────────────────────

module "frontend" {
  source = "./modules/frontend"

  project_id = var.project_id
  region     = var.region
  agent_url  = module.agent.cloud_run_service_url
}
