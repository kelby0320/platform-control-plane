# Production Dockerfile - Slim image with only the application binary
FROM rust:1.91 AS builder

WORKDIR /usr/src/platform-control-plane

# Install build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy proto files needed for build (create platform-apis/proto structure expected by build.rs)
COPY platform-apis/ ./platform-apis/

# Copy all crates
COPY backend/crates/domain/ ./backend/crates/domain/
COPY backend/crates/infra/ ./backend/crates/infra/
COPY backend/crates/platform-api/ ./backend/crates/platform-api/

# Database URL needed by sqlx
ENV DATABASE_URL="postgres://postgres:postgres@host.docker.internal:15002/platform_db"

# Build the application in release mode
RUN cargo build --release --bin platform-api

# Final stage: slim runtime image
FROM debian:trixie-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy only the binary from builder
COPY --from=builder /usr/src/platform-control-plane/target/release/platform-api /usr/local/bin/platform-api

# Run as non-root user
RUN useradd -r -s /bin/false platform-api && \
    chown platform-api:platform-api /usr/local/bin/platform-api

USER platform-api

CMD ["platform-api"]
