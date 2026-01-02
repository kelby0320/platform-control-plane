# Reviewer Instructions — PCP Architecture Review

You are reviewing a proposed or completed change in the PCP repository.

Your task is to identify architectural, layering, or dependency issues.

---

## Review Checklist

### Crate Dependencies
- Does `domain` depend on `infra` or `platform-api`? (❌)
- Does `infra` depend on `platform-api`? (❌)
- Are new dependencies justified and minimal?

---

### Layer Responsibilities
- Is business logic leaking into `infra` or `platform-api`?
- Are infrastructure concerns leaking into `domain`?
- Are API concerns confined to `platform-api`?

---

### Ports & Adapters
- Are traits defined in `domain`?
- Are implementations located in `infra`?
- Is dependency inversion preserved?

---

### Rust Code Quality
- Are errors explicit and meaningful?
- Are `unwrap` / `expect` avoided in production code?
- Are public APIs intentional and documented?

---

### Tests
- Are new behaviors covered by tests?
- Are tests placed in appropriate crates?
- Are tests overly coupled to infrastructure?

---

## Output Format

Provide:
1. **Critical violations** (must fix)
2. **Warnings** (should fix)
3. **Suggestions** (optional)

Reference files and symbols precisely.
Do not propose refactors beyond the original scope.