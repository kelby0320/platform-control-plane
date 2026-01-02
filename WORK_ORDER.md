## Goal
(Describe the desired behavior or API change.)

---

## Scope (Allowed Changes)
(List the narrowest possible scope.)

- backend/crates/domain/...
- backend/crates/infra/...
- backend/crates/platform-api/...

---

## Non-Scope (Must NOT Change)
- No changes to:
  - unrelated crates or modules
  - dependency directions
  - public APIs unless explicitly stated

---

## Architectural Constraints
- Follow AGENTS.md strictly
- Domain crate must remain infrastructure-agnostic
- Infrastructure implements domain traits
- API concerns live only in platform-api

---

## Interfaces / Contracts
(List or define traits, structs, endpoints, or function signatures.)

Example:
```rust
pub trait ArtifactStore {
    fn put(&self, artifact: Artifact) -> Result<ArtifactId, StoreError>;
}
```

---

## Data Flow
(Briefly describe how data moves across layers.)

Example:
HTTP → DTO → domain service → trait → infra adapter → result → DTO

---

## Tests

* Unit tests:
  * location:
  * behavior:
* Integration tests (if applicable):

---

## Acceptance Checklist
- [ ] cargo fmt
- [ ] cargo clippy (no warnings)
- [ ] cargo test
- [ ] No crate boundary violations
- [ ] No unnecessary refactors

---

## Instructions to Agent
* Plan first and present the plan before coding
* Ask before touching files outside scope
* Optimize for minimal, correct changes
* Stop and ask if AGENTS.md would be violated