output "cloud_run_url" {
  description = "The URL of the deployed API — use this in launcher.json"
  value       = google_cloud_run_v2_service.api.uri
}

output "artifact_registry_image_base" {
  description = "Base image path for docker push"
  value       = "${var.region}-docker.pkg.dev/${var.project_id}/auto-platform/api"
}

output "api_master_key_secret" {
  description = "Retrieve with: gcloud secrets versions access latest --secret=API_MASTER_KEY"
  value       = "API_MASTER_KEY"
}