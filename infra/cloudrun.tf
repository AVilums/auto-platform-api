# The default compute service account Cloud Run uses
data "google_compute_default_service_account" "default" {}

# Grant it access to read the API key secret
resource "google_secret_manager_secret_iam_member" "api_key_access" {
  secret_id = google_secret_manager_secret.api_master_key.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${data.google_compute_default_service_account.default.email}"
}

resource "google_cloud_run_v2_service" "api" {
  name     = "auto-platform-api"
  location = var.region

  template {
    service_account = data.google_compute_default_service_account.default.email

    scaling {
      min_instance_count = 0
      max_instance_count = 2
    }

    containers {
      image = "${var.region}-docker.pkg.dev/${var.project_id}/auto-platform/api:${var.image_tag}"

      ports {
        container_port = 8080
      }

      env {
        name  = "HOST"
        value = "0.0.0.0" # required for Cloud Run to receive traffic
      }

      env {
        name  = "PORT"
        value = "8080"
      }

      env {
        name  = "RUST_LOG"
        value = "info"
      }

      # API key injected from Secret Manager — never stored in plain text
      env {
        name = "API_MASTER_KEY"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.api_master_key.secret_id
            version = "latest"
          }
        }
      }
    }
  }

  depends_on = [
    google_secret_manager_secret_iam_member.api_key_access,
  ]
}

# Make the service publicly reachable (authentication is handled by the
# X-API-Key header middleware in the application itself).
resource "google_cloud_run_v2_service_iam_member" "public" {
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.api.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
