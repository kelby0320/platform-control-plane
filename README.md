# Platform Control Plane

The Platform Control Plane is the backend foundation of the larger Glass Fort AI system. It exposes APIs for identity, user management, chat sessions, billing, usage, and orchestrating requests to the AI Service Plane. This repository contains the Rust-based backend services and supporting infrastructure that power the platform.

This project is designed as a multi-crate Rust workspace and will eventually include multiple components, such as:

* platform-api — HTTP API built with Axum
* Additional microservices (future)
* Shared libraries (future)
* Frontend applications (Next.js) in apps/ (future)

## Getting Started

### Prerequisites
* Rust (latest stable)
* Just - https://github.com/casey/just

## Running the API

From the repository root:
```bash
just run
```

Or manually:
```bash
cargo run --bin platform-api
```

The API will start on http://localhost:8000

## Architecture Overview

The Platform Control Plane is responsible for:
* User identity and authentication
* User/account management
* Chat session state
* Billing and subscription data
* Usage and metering
* Forwarding inference requests to the AI Service Plane
* Maintaining platform-level configuration and orchestration
* It serves as the “backend brain” of the overall system, with the AI Service Plane handling LLM execution, embeddings, RAG, and tool calls.

As the project matures, this README will expand with:

* API documentation
* Sequence diagrams
* Architectural decision records
* Database schema documentation
* Deployment instructions (Docker, Kubernetes, etc.)

## Future Roadmap (High-Level)

* Add identity & auth modules
* Add chat session API
* Integrate with AI Service Plane inference endpoints
* Add billing & usage services
* Add internal orchestration logic
* Add Next.js frontends under apps/
* Add full CI/CD pipelines for both backend + frontend
* Provide containerization (Dockerfiles and deploy scripts)