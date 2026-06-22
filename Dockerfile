# ── Stage 1: builder ──────────────────────────────────────────────────────────
FROM rust:1.81-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependency compilation first (Cargo layer-caching trick)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build --release && rm -rf src

# Build the real binary
COPY . .
RUN touch src/main.rs && cargo build --release

# ── Stage 2: runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Non-root user for security
RUN useradd -ms /bin/bash appuser
USER appuser
WORKDIR /app

COPY --from=builder /app/target/release/auto-platform-api /app/auto-platform-api

EXPOSE 8080

ENTRYPOINT ["/app/auto-platform-api"]
