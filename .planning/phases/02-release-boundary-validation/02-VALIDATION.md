---
phase: 02
slug: release-boundary-validation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-07-29
updated: 2026-07-29
---

# Phase 2 — Validation Strategy

> Execution-time Nyquist contract. Pending rows must be replaced with measured evidence before phase completion.

## Test Infrastructure

| Property | Value |
|----------|-------|
| Framework | Vitest 4.1.10 + Vue Test Utils 2.4.11 + happy-dom 20.11.1 (Wave 0 install) |
| Config file | `vite.config.ts` (Wave 0 adds `test.environment`) |
| Quick run command | `pnpm test -- src/panes/SearchPane.spec.ts` |
| Full suite command | `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml` |
| Estimated runtime | Pending measurement during execution |

## Sampling Rate

- After every implementation task: run its listed automated command.
- After each wave: run the full suite applicable to artifacts then present.
- Before phase verification: all runnable gates must be green.
- No watch-mode command is permitted.

## Requirements-to-Test Map

| Requirement | Observable behavior/evidence | Automated command | Artifact | Status |
|-------------|------------------------------|-------------------|----------|--------|
| UI-01 | Failed pagination retains prior page/results; success publishes page/results together. | `pnpm test -- src/panes/SearchPane.spec.ts` | `src/panes/SearchPane.spec.ts` | pending — Wave 0 |
| VAL-01 | Focused/full tests, frontend build, Rust check, and per-task evidence are recorded. | `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml` | `02-VALIDATION.md`, `02-VERIFICATION.md` | pending |
| VAL-01 / PHASE-1 audit closure | Phase 1 receives explicitly retrospective Nyquist evidence grounded in its existing summary and verification. | `Test-Path .planning/phases/01-stabilization-cleanup/01-VALIDATION.md` | `01-VALIDATION.md` | pending |
| REL-01 | Deferred patch is isolated to an immutable stash object and the clean tested candidate SHA excludes it. | Plan 03 candidate/stash commands | `02-VALIDATION.md` | pending — release gate |
| REL-01 | Restored binary diff SHA-256 matches exactly, four symbols exist individually, file is unstaged, and stash remains resolvable. | Plan 03 restoration commands | working diff + stash object | pending — restoration gate |

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Automated Command | Status |
|---------|------|------|-------------|-------------------|--------|
| 02-01-01 | 01 | 1 | VAL-01 | `pnpm exec vitest run --passWithNoTests` | pending |
| 02-01-02 | 01 | 1 | UI-01 | `pnpm test -- src/panes/SearchPane.spec.ts` | pending |
| 02-02-01 | 02 | 2 | VAL-01 | `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml` | pending |
| 02-02-02 | 02 | 2 | VAL-01 | `Test-Path .planning/phases/01-stabilization-cleanup/01-VALIDATION.md` | pending |
| 02-03-01 | 03 | 3 | REL-01 | stash object/type/path and binary SHA-256 checks | pending |
| 02-03-02 | 03 | 3 | REL-01 | clean full gates plus candidate SHA symbol-negative scan | pending |
| 02-03-03 | 03 | 3 | REL-01, VAL-01 | restored SHA-256, four symbols, unstaged and stash-resolvable checks | pending |

## Wave 0 Requirements

- [ ] Install pinned Vitest, Vue Test Utils, and happy-dom dependencies.
- [ ] Add non-watch `pnpm test` and Vitest configuration.
- [ ] Create `src/panes/SearchPane.spec.ts` with failure and success cases.
- [ ] Confirm the focused command exits successfully.

## Pending Release-Boundary Gates

- [ ] Record pre-isolation binary diff SHA-256 and four expected symbols.
- [ ] Persist immutable stash object ID; require commit type and exactly `src/AppContent.vue`.
- [ ] Require stash binary diff SHA-256 equality.
- [ ] Run tests/build/cargo from a clean tree and record full candidate commit SHA.
- [ ] Candidate `AppContent.vue` lacks all four symbols; Phase 2 creates no tag.
- [ ] Apply immutable stash object; on conflict/non-zero status retain stash and stop.
- [ ] Restored binary diff SHA-256 is identical; each symbol exists individually.
- [ ] `src/AppContent.vue` is unstaged and stash object remains resolvable.
- [ ] Formal annotated `v1.0` remains pending `$gsd-complete-milestone`.

## Validation Sign-Off

- [ ] Every task has automated verification.
- [ ] Sampling continuity has no three-task gap.
- [ ] Wave 0 is complete.
- [ ] No watch-mode flags are used.
- [ ] Full suite and all release gates are green.
- [ ] Set `nyquist_compliant: true` and `status: complete` only after evidence is recorded.

**Approval:** pending
