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
**Plans:** 3/3 plans complete

Plans:

**Wave 1**
- [x] 02-01-PLAN.md — Add focused Vue test infrastructure and enforce success-only pagination state publication.

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 02-02-PLAN.md — Run full automated gates and produce Nyquist plus phase-verification evidence.

**Wave 3** *(blocked on Wave 2 completion)*
- [x] 02-03-PLAN.md — Record the clean tested release-candidate SHA, restore deferred debounce work, and re-run milestone audit readiness; formal v1.0 tagging remains with milestone completion.

Cross-cutting constraints:
- Preserve the user-owned `src/AppContent.vue` debounce diff; never stage, commit, overwrite, or include it in the v1.0 candidate boundary.
- Leave formal `v1.0` tag creation to `$gsd-complete-milestone`.

## Backlog

### Phase 999.1: Deferred UX polish after usage period (BACKLOG)

**Goal:** Capture experience improvements to revisit after using the app for a while.
**Requirements:** TBD
**Plans:** 0 plans

Plans:
- [ ] Add an auto-close countdown progress bar to popup notifications.
- [ ] Debounce settings persistence when adjusting the number of Ehentai favorites to download, so repeated changes do not trigger repeated "saved successfully" messages.
