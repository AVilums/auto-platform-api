# auto-platform-api

A lightweight Rust backend that receives telemetry batches from
[automation-launcher](../automation-launcher) and logs them as structured JSON.
Designed to run on **Google Cloud Run**, provisioned with Terraform.

---

## Architecture

```
automation-launcher
      │
      │  POST /v1/events   (X-API-Key header)
      ▼
auto-platform-api  ──►  tracing (structured JSON)  ──►  Cloud Logging
```

- **No database** — events are logged to stdout; Cloud Run ships those logs
  straight to Google Cloud Logging.
- **Auth** — every request to `/v1/events` must carry an `X-API-Key` header
  matching the `API_MASTER_KEY` environment variable.
- **Modular** — adding persistence later is a matter of injecting a storage
  trait into the handler; no rewiring of routes or middleware.

### HTTP surface

| Method | Path          | Auth | Description                        |
|--------|---------------|------|------------------------------------|
| GET    | `/health`     | No   | Liveness probe for Cloud Run       |
| POST   | `/v1/events`  | Yes  | Ingest a batch of telemetry events |

#### `POST /v1/events` payload

```json
{
  "events": [
    {
      "id": "uuid-string",
      "timestamp": "2024-06-01T10:00:00Z",
      "event_type": "ExecutionSuccess",
      "tool_name": "data-processor",
      "tool_version": "1.2.0",
      "status": "ok",
      "details": null
    }
  ]
}
```

`event_type` mirrors the launcher's `EventType` enum.  Unknown variants are
accepted gracefully (they deserialise to `Unknown`) so the API never rejects
events from a newer launcher version.

---

## Local development

### Prerequisites

- Rust (stable, via [rustup](https://rustup.rs))
- Docker (optional, for the Docker workflow)

### Run directly

```bash
# 1 — create a local env file
cat > .env.local <<'EOF'
API_MASTER_KEY=localdev-secret
HOST=127.0.0.1
PORT=8080
RUST_LOG=debug
EOF

# 2 — run
API_MASTER_KEY=localdev-secret cargo run

# 3 — test the health endpoint
curl http://localhost:8080/health

# 4 — send a telemetry batch
curl -X POST http://localhost:8080/v1/events \
  -H "Content-Type: application/json" \
  -H "X-API-Key: localdev-secret" \
  -d '{"events":[{"id":"1","timestamp":"2024-01-01T00:00:00Z","event_type":"LauncherStartup","tool_name":null,"tool_version":null,"status":null,"details":null}]}'
```

### Run with Docker

```bash
# Production image
docker compose up --build

# Local dev (cargo run inside the container, mounts source)
docker compose -f docker-compose.yml -f docker-compose.local.yml up
```

---

## Tests

```bash
API_MASTER_KEY=test cargo test
```

Unit tests live alongside their modules (`src/config.rs`, `src/models.rs`).
Integration tests are in `tests/events_api.rs` and use
[`axum-test`](https://crates.io/crates/axum-test) — no network, no ports, no
external services required.

---

## Environment variables

| Variable         | Required | Default     | Description                        |
|------------------|----------|-------------|------------------------------------|
| `API_MASTER_KEY` | ✅        | —           | Shared secret for `X-API-Key` auth |
| `HOST`           | No       | `127.0.0.1` | Bind address (`0.0.0.0` on Cloud Run) |
| `PORT`           | No       | `8080`      | Bind port                          |
| `RUST_LOG`       | No       | `info`      | [`tracing-subscriber`] filter      |

---

## Deploying to GCP

### One-time setup

```bash
# Get GCP CLI (Windows)
(New-Object Net.WebClient).DownloadFile("https://dl.google.com/dl/cloudsdk/channels/rapid/GoogleCloudSDKInstaller.exe", "$env:Temp\GoogleCloudSDKInstaller.exe")
& $env:Temp\GoogleCloudSDKInstaller.exe

gcloud init

# Get Terraform (Windows)
winget install -e --id Hashicorp.Terraform

# Authenticate
gcloud auth login
gcloud config set project YOUR_PROJECT_ID

# Allow ADC
gcloud auth application-default login

# Create the Terraform state bucket
gcloud storage buckets create gs://YOUR_PROJECT_ID-tfstate \
  --project=YOUR_PROJECT_ID \
  --location=europe-north1 \
  --uniform-bucket-level-access

# Initialise Terraform
cd infra
terraform init

# Review and apply (uses Application Default Credentials)
terraform apply -var="project_id=YOUR_PROJECT_ID"
```

### Deploy a new version

```bash
# Build and push the image
IMAGE="europe-north1-docker.pkg.dev/YOUR_PROJECT_ID/auto-platform/api"
docker build -t "$IMAGE:$GIT_SHA" .
docker push "$IMAGE:$GIT_SHA"

# Roll out
cd infra
terraform apply \
  -var="project_id=YOUR_PROJECT_ID" \
  -var="image_tag=$GIT_SHA"
```

The Cloud Run service URL is printed by Terraform as `cloud_run_url`.

### Retrieve the API key

```bash
gcloud secrets versions access latest --secret=API_MASTER_KEY
```

Configure this value in the launcher's telemetry remote endpoint settings.

---

## Infra overview (`infra/`)

| File            | Purpose                                              |
|-----------------|------------------------------------------------------|
| `main.tf`       | Provider, backend, API enablement                    |
| `variables.tf`  | Input variables (`project_id`, `region`, `image_tag`)|
| `cloudrun.tf`   | Cloud Run service + IAM + Secret Manager binding     |
| `registry.tf`   | Artifact Registry repository for Docker images       |
| `secrets.tf`    | `API_MASTER_KEY` secret (auto-generated)             |
| `network.tf`    | Placeholder for future VPC resources                 |
| `outputs.tf`    | `cloud_run_url`, `artifact_registry_image_base`      |

---

## Project structure

```
src/
  lib.rs          — public module re-exports (used by integration tests)
  main.rs         — entry point: loads config, sets up tracing, binds server
  config.rs       — Config struct, loaded from environment variables
  models.rs       — TelemetryEvent, EventBatch (mirrors launcher types)
  error.rs        — ApiError with axum IntoResponse
  auth.rs         — X-API-Key middleware
  routes/
    mod.rs        — Router builder (public + protected route groups)
    health.rs     — GET /health
    events.rs     — POST /v1/events
tests/
  events_api.rs   — Integration tests (axum-test, no external services)
infra/            — Terraform (GCP Cloud Run + Artifact Registry + Secrets)
```
