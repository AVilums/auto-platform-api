# Terraform creates the secret shell; the value is populated automatically
# by generating a random key — no manual step needed.

resource "google_secret_manager_secret" "api_master_key" {
  secret_id = "API_MASTER_KEY"

  replication {
    auto {}
  }

  depends_on = [google_project_service.apis]
}

# Generate a cryptographically random key — no manual openssl step needed.
resource "random_password" "api_master_key" {
  length  = 64
  special = false
}

resource "google_secret_manager_secret_version" "api_master_key" {
  secret      = google_secret_manager_secret.api_master_key.id
  secret_data = random_password.api_master_key.result
}
