FROM rust:1.91 AS builder
WORKDIR /usr/src/platform-control-plane
COPY backend/crates/platform-api/ ./backend/crates/platform-api
COPY backend/crates/domain/ ./backend/crates/domain
COPY backend/crates/infra/ ./backend/crates/infra
COPY platform-apis/proto/ ./platform-apis/proto
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev libpq-dev
ENV DATABASE_URL="postgres://postgres:postgres@host.docker.internal:15002/platform_db"
RUN cargo install --path ./backend/crates/platform-api

FROM debian:trixie-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/platform-api /usr/local/bin/platform-api
CMD ["platform-api"]
