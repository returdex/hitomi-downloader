# Roadmap

## Active Milestone

### Milestone: Stabilization

Improve day-to-day app quality with small, low-risk fixes.

### Phase 1: Stabilization cleanup

**Goal:** Clean up low-risk stabilization issues after the recent E-Hentai favorites work.
**Requirements:** TBD
**Plans:** 1 plan

Plans:
- [x] 01-01 Stabilization cleanup (Wave 1): resolve date i18n TODOs, add focused user-visible error feedback, and run standard verification.

### Phase 2: Release boundary and validation closure

**Goal:** Establish an auditable v1.0 release boundary, fix pagination error-state consistency, and complete automated validation evidence.
**Requirements:** REL-01, UI-01, VAL-01
**Gap Closure:** Closes gaps from the v1.0 milestone audit.
**Plans:** 0 plans

Planned work:
- [ ] Preserve the uncommitted settings-save debounce for the next milestone while excluding it from the v1.0 release tag.
- [ ] Keep pagination state aligned with displayed results when page loading fails.
- [ ] Add automated validation for the affected UI state and produce Nyquist validation evidence.
- [ ] Re-run the milestone audit against the clean v1.0 release boundary.

## Backlog

### Phase 999.1: Deferred UX polish after usage period (BACKLOG)

**Goal:** Capture experience improvements to revisit after using the app for a while.
**Requirements:** TBD
**Plans:** 0 plans

Plans:
- [ ] Add an auto-close countdown progress bar to popup notifications.
- [ ] Debounce settings persistence when adjusting the number of Ehentai favorites to download, so repeated changes do not trigger repeated "saved successfully" messages.
