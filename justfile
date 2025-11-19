set dotenv-load := true

default: build

# ----- Dev commands -----

build:
    cargo build

test:
    cargo test

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets --all-features -- -D warnings

run:
    cargo run --bin platform-api

# ----- Database/migrations -----

db-create:
    sqlx database create

db-migrate:
    sqlx migrate run --source backend/crates/infra/migrations

# ----- Docker -----

docker-build: docker-build-platform-api

docker-build-platform-api:
    docker build -t platform-api:latest -f docker/platform-api.Dockerfile .

# ----- Docker Compose -----

compose-up:
    docker compose up -d

compose-down:
    docker compose down

compose-logs:
    docker compose logs -f