FROM rust:1.91 as builder
WORKDIR /usr/src/platform-control-plane
COPY backend/crates/platform-api/ ./platform-api
COPY backend/crates/domain/ ./domain
COPY backend/crates/infra/ ./infra
RUN cargo install --path ./platform-api

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/platform-api /usr/local/bin/platform-api
CMD ["platform-api"]
