# Requirements: v1.0 Stabilization

**Milestone:** v1.0
**Coverage:** 1/4 requirements complete
**Last updated:** 2026-07-29 after milestone gap planning

## Completed Requirements

- [x] **PHASE-1** — Complete the low-risk stabilization cleanup without
  pulling deferred UX features into the committed Phase 1 scope.
  - D-01/D-05: Render comic dates through a locale-aware shared helper.
  - D-02/D-06/D-07: Keep settings debounce, notification countdown,
    popularity sorting, and unrelated Rust cleanup out of Phase 1.
  - D-03: Show visible feedback for important direct user-action failures.
  - D-04: Do not duplicate existing E-Hentai notification paths.
  - D-08/D-09: Pass the frontend and Rust checks and review notification
    noise.

## Gap Closure Requirements

- [ ] **REL-01** — Establish a clean v1.0 release boundary that excludes the
  uncommitted settings-save debounce while preserving that work for the next
  milestone.
- [ ] **UI-01** — Keep pagination state and displayed results consistent when
  loading a requested page fails.
- [ ] **VAL-01** — Add automated validation for the affected UI behavior and
  produce Nyquist validation evidence for the milestone.

## Out of Scope for v1.0

- Notification auto-close countdown progress bar.
- Settings-save debounce as a shipped v1.0 capability; the current
  uncommitted implementation is reserved for the next milestone.
- Search sorting by popularity.
- Unrelated Rust TODO and warning cleanup.

## Traceability

| Requirement | Description | Phase | Status |
|---|---|---:|---|
| PHASE-1 | Low-risk stabilization cleanup | 1 | Complete |
| REL-01 | Clean v1.0 release boundary | 2 | Pending |
| UI-01 | Pagination failure-state consistency | 2 | Pending |
| VAL-01 | Automated and Nyquist validation evidence | 2 | Pending |

## Audit Sources

- `.planning/phases/01-stabilization-cleanup/01-01-PLAN.md`
- `.planning/phases/01-stabilization-cleanup/01-01-SUMMARY.md`
- `.planning/phases/01-stabilization-cleanup/01-VERIFICATION.md`
- `.planning/v1.0-MILESTONE-AUDIT.md`
