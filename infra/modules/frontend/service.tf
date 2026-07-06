resource "google_cloud_run_v2_service" "frontend" {
  name                = "frontend"
  location            = var.region
  project             = var.project_id
  deletion_protection = false
  ingress             = "INGRESS_TRAFFIC_ALL"

  template {
    containers {
      # Placeholder replaced by CI/CD on every push to main.
      image = "us-docker.pkg.dev/cloudrun/container/hello"

      # Dioxus fullstack server listens on 8080 by default (set via ENV in Dockerfile).
      ports {
        container_port = 8080
      }

      env {
        name  = "AGENT_URL"
        value = var.agent_url
      }

      resources {
        limits = {
          cpu    = "1"
          memory = "512Mi"
        }
      }
    }

    scaling {
      min_instance_count = 0
      max_instance_count = 5
    }
  }

  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }

  # CI/CD updates the image — Terraform only owns service config.
  lifecycle {
    ignore_changes = [
      template[0].containers[0].image,
    ]
  }
}

resource "google_cloud_run_v2_service_iam_member" "public_invoker" {
  count    = var.allow_unauthenticated_invocations ? 1 : 0
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.frontend.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
