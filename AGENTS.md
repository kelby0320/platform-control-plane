# AGENTS.md — Platform Control Plane (PCP)

This file defines the rules and expectations for AI agents working in the
Platform Control Plane (PCP) repository.

PCP is a Rust codebase organized as a **layered system with strict crate
boundaries**. Architectural correctness is mandatory.

---

## Repository Structure

platform-control-plane/
└── backend/crates/
    ├── domain/
    ├── infra/
    └── platform-api/

---

## Architectural Overview

### domain (Core / Business Layer)
- Contains **business logic only**
- Defines:
  - domain models
  - domain services
  - domain errors
  - traits (ports) required by the domain
- Must be **pure Rust logic**
- **Must not depend on**:
  - infra
  - platform-api
  - databases, HTTP, auth, config, environment variables
- No serialization, persistence, or transport logic

---

### infra (Infrastructure Layer)
- Depends on `domain`
- Implements traits (ports) defined in `domain`
- Responsible for:
  - database access
  - external services
  - persistence
  - caches, queues, artifact storage, etc.
- Must not contain business rules
- Must not expose HTTP or API concerns

---

### platform-api (API / Boundary Layer)
- Depends on `domain` and `infra`
- Owns:
  - HTTP / gRPC endpoints
  - auth & identity integration
  - request / response DTOs
  - serialization and validation
- Coordinates application flow
- Must not contain business logic

---

## Dependency Rules (Hard Constraints)

These rules are **strict and enforced**:

- `domain`
  - ❌ must not depend on `infra`
  - ❌ must not depend on `platform-api`

- `infra`
  - ✅ may depend on `domain`
  - ❌ must not depend on `platform-api`

- `platform-api`
  - ✅ may depend on `domain`
  - ✅ may depend on `infra`

Breaking these rules is a correctness failure.

---

## Ports & Adapters

- Traits (ports) are defined in `domain`
- Implementations (adapters) live in `infra`
- Dependency inversion must be preserved
- Wiring occurs in `platform-api` or `infra`

---

## Tooling & Quality Gates

Agents must ensure the following pass before work is considered complete:

- Formatting:
  - cargo fmt
- Linting:
  - cargo clippy --all-targets --all-features -- -D warnings
- Tests:
  - cargo test

Warnings are treated as errors unless explicitly approved.

---

## Coding Conventions

- Follow idiomatic Rust
- Prefer explicit types and error enums
- Avoid `unwrap` / `expect` in non-test code
- Use `Result` and domain-specific errors
- Keep modules small and focused
- No architectural shortcuts for convenience

---

## Agent Workflow (Required)

1. **Plan first**
 - Identify crates and modules to change
 - Describe data flow across layers
 - List traits and implementations involved

2. **Ask before proceeding** if the plan:
 - crosses crate boundaries unexpectedly
 - introduces new dependencies
 - modifies public APIs

3. **Implement minimal diffs**
 - Avoid unrelated refactors
 - Do not move code across crates unless instructed

4. **Verify**
 - Run fmt, clippy, and tests
 - Re-check dependency direction

---

## When to Stop and Ask

Agents must stop and ask if:
- a change violates crate boundaries
- domain logic appears to belong elsewhere
- new traits or adapters are required but unclear
- scope expands beyond the work order

---

## Guiding Principle

> Architectural correctness and clarity outweigh speed.
> Small, correct changes are preferred over large refactors.
