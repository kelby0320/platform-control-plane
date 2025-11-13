default: build

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