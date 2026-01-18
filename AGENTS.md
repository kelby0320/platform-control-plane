# AGENTS.md — Platform Control Plane (PCP)

This file defines **non-negotiable rules** for AI agents working in the
Platform Control Plane (PCP) repository.

PCP is a Rust codebase organized as a **layered system with strict crate
boundaries**. Architectural correctness is mandatory.

If any rule in this document is violated, the work is considered incorrect
even if it compiles or tests pass.

---

## Repository Structure

```
platform-control-plane/
└── backend/crates/
    ├── domain/
    ├── infra/
    └── platform-api/
```

This structure is intentional and must be preserved.

---

## Architectural Layers

### domain (Core / Business Layer)

The `domain` crate contains **pure business logic**.

Allowed contents:
- domain models
- domain services
- domain errors
- traits (ports) required by the domain

Hard constraints:
- MUST NOT depend on `infra`
- MUST NOT depend on `platform-api`
- MUST NOT depend on databases, HTTP, auth, config, or environment variables
- MUST NOT contain serialization or transport logic

This crate must remain pure Rust logic.

---

### infra (Infrastructure Layer)

The `infra` crate provides implementations of domain-defined traits.

Responsibilities:
- database access
- external services
- persistence
- caches, queues, artifact storage

Constraints:
- MAY depend on `domain`
- MUST NOT depend on `platform-api`
- MUST NOT contain business rules
- MUST NOT expose HTTP or API concerns

---

The `platform-api` crate coordinates application flow.

Responsibilities:
- HTTP and gRPC endpoints
- auth and identity integration
- request/response DTOs
- validation and serialization
- wiring of domain and infra components

Constraints:
- MAY depend on `domain`
- MAY depend on `infra`
- MUST NOT contain business logic

---

## Dependency Rules (Hard Constraints)

These rules are strict and enforced:

- `domain`
  - MUST NOT depend on `infra`
  - MUST NOT depend on `platform-api`

- `infra`
  - MAY depend on `domain`
  - MUST NOT depend on `platform-api`

- `platform-api`
  - MAY depend on `domain`
  - MAY depend on `infra`

Breaking dependency direction is a correctness failure.

---

## Ports & Adapters

- Traits (ports) are defined in `domain`
- Implementations (adapters) live in `infra`
- Dependency inversion must be preserved
- Wiring occurs in `platform-api` (or infra initialization code)

---

## Tooling Rules (Strict)

Agents MUST use Cargo tooling correctly.

### Formatting
```sh
cargo fmt
```

### Linting
```sh
cargo clippy --all-targets --all-features -- -D warnings
```

### Tests
```sh
cargo test
```

Warnings are treated as errors unless explicitly approved.

---

## Dependency Management (Critical Rule)

- DO NOT manually edit Cargo.toml to add dependencies
- ALWAYS add dependencies using Cargo commands

Required usage:
```sh
cargo add <crate>
cargo add <crate> --features <feature>
```

Rationale:
- Manual edits frequently introduce incorrect versions
- Cargo is the authority on compatible versions

Manual dependency edits are considered a tooling violation.

---

## Coding Conventions

- Follow idiomatic Rust
- Prefer explicit types and domain-specific error enums
- Avoid `unwrap` / `expect` in non-test code
- Use `Result` for fallible operations
- Keep modules small and focused
- No architectural shortcuts for convenience

---

## Agent Workflow (Required)

1. **Plan first**
 - Identify crates and modules to change
 - Describe data flow across layers
 - Identify traits (ports) and implementations involved

2. **Stop and ask** before proceeding if the plan::
 - crosses crate boundaries unexpectedly
 - introduces new dependencies
 - modifies public APIs or request/response types

3. **Implement minimal diffs**
 - Change only what the work order requires
 - Avoid unrelated refactors
 - Do not move code across crates without instruction

4. **Verify**
 - Run `cargo fmt`, `cargo clippy`, and `cargo test`
 - Re-check dependency direction

---

## Mandatory Stop Conditions

Agents MUST STOP and report if:
- a change violates crate dependency rules
- domain logic appears in infra or platform-api
- a new dependency is required but its placement is unclear
- scope expands beyond the work order

---

## Guiding Principle

> Architectural correctness and clarity outweigh speed.
> Small, correct changes are preferred over large refactors.
