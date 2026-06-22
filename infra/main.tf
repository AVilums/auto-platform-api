terraform {
  required_version = ">= 1.7"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
  }

  # Store state in GCS so it's shared across machines and CI.
  # Create the bucket manually first:
  #   gsutil mb -p YOUR_PROJECT_ID gs://YOUR_PROJECT_ID-tfstate
  backend "gcs" {
    bucket = "YOUR_PROJECT_ID-tfstate"
    prefix = "auto-platform"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# Enable all required GCP APIs in one place.
resource "google_project_service" "apis" {
  for_each = toset([
    "run.googleapis.com",
    "artifactregistry.googleapis.com",
    "secretmanager.googleapis.com",
  ])

  service            = each.value
  disable_on_destroy = false # keep APIs enabled even after `terraform destroy`
}
