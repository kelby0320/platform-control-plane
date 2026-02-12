# Development Dockerfile - Contains codebase, tooling, and supports migrations
FROM rust:1.91-slim

# Install development dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    libpq-dev \
    pkg-config \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --version 0.8.6

# Set working directory
WORKDIR /usr/src/platform-control-plane

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy all crates (domain, infra, platform-api)
COPY backend/crates/domain/ ./backend/crates/domain/
COPY backend/crates/infra/ ./backend/crates/infra/
COPY backend/crates/platform-api/ ./backend/crates/platform-api/

# Copy platform-apis
COPY platform-apis/ ./platform-apis/

# Database URL needed by sqlx
ENV DATABASE_URL="postgres://postgres:postgres@host.docker.internal:15002/platform_db"

# Build the application
RUN cargo build --bin platform-api

# Set up environment for sqlx-cli migrations
ENV SQLX_OFFLINE=false

# Default command runs the application
# For migrations, use: sqlx migrate run
CMD ["cargo", "run", "--bin", "platform-api"]
