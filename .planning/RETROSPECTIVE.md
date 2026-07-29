# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.0 — Stabilization

**Shipped:** 2026-07-29
**Phases:** 2 | **Plans:** 4 | **Tasks:** 10

### What Was Built

- Localized comic dates and focused visible feedback for direct user actions.
- E-Hentai favorites import, persisted settings, startup checking, and cookie-field migration.
- Transactional pagination state with Vitest regression coverage.
- Reproducible frontend/Rust validation, Nyquist evidence, and a clean release candidate.

### What Worked

- Small phase boundaries kept stabilization work reviewable.
- Requirement IDs, verification reports, and candidate-boundary checks made the release auditable.
- Exact-path staging and immutable stash hashes protected user-owned work in a dirty tree.

### What Was Inefficient

- The first milestone audit lacked a requirements baseline and required a gap-closure phase.
- GSD's quick-task audit expected canonical `SUMMARY.md` names, which required compatibility entry files.
- The local Git version required a first-parent diff workaround for stash merge commits.

### Patterns Established

- Publish related UI state only after the asynchronous command succeeds.
- Keep historical validation evidence explicit about retrospective versus current measurements.
- Reserve formal release tags for milestone completion after archive commits.

### Key Lessons

1. Create and maintain a requirements traceability table before the first milestone audit.
2. Treat user-owned dirty changes as explicit release-boundary assets with byte-level recovery evidence.
3. Use canonical GSD artifact names alongside descriptive historical filenames.

### Cost Observations

- Model mix: balanced profile
- Sessions: multiple planning and execution sessions
- Notable: the release-boundary safety plan caught tooling-specific stash behavior before any destructive restore step.

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | multiple | 2 | Added audit-driven gap closure and candidate-boundary verification |

### Cumulative Quality

| Milestone | Tests | Coverage | Zero-Dep Additions |
|-----------|-------|----------|-------------------|
| v1.0 | 2 focused + full suite | audit passed | Vitest/Vue Test Utils/happy-dom |

### Top Lessons (Verified Across Milestones)

1. Narrow phases and explicit verification evidence reduce release ambiguity.
2. Preserve user-owned work through path-scoped, byte-checked recovery protocols.
