set dotenv-load := true

default: build

# ----- Dev commands -----

build:
    docker compose up platform-postgres -d
    cargo build
    docker compose down

test:
    docker compose up platform-postgres -d
    cargo test
    docker compose down

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets --all-features -- -D warnings

run:
    cargo run --bin platform-api

# ----- Database/migrations -----

db-migrate:
    sqlx migrate run --source backend/crates/infra/migrations

# ----- Docker -----

docker-build: docker-build-dev

docker-build-dev:
    docker compose up platform-postgres -d
    docker build -t platform-api-dev:latest -f docker/platform-api.dev.Dockerfile .
    docker compose down platform-postgres

docker-build-prod:
    docker compose up platform-postgres -d
    docker build -t platform-api:latest -f docker/platform-api.prod.Dockerfile .
    docker compose down platform-postgres

# ----- Docker Compose -----

compose-up:
    docker compose up -d

compose-down:
    docker compose down

compose-logs:
    docker compose logs -f