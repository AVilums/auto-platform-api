resource "google_artifact_registry_repository" "auto_platform" {
  location      = var.region
  repository_id = "auto-platform"
  format        = "DOCKER"

  depends_on = [google_project_service.apis]
}