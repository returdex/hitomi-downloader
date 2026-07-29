# Phase 1: Stabilization cleanup - Context

**Gathered:** 2026-05-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Clean up low-risk stabilization issues after the recent E-Hentai favorites work. This phase covers user-visible error feedback, small UI class/formatting defects, date i18n TODOs, and build/type verification. It does not add new product capabilities or pull deferred backlog UX work into the active milestone.

</domain>

<decisions>
## Implementation Decisions

### Cleanup Scope
- **D-01:** Use the "include small TODOs" scope: handle obvious low-risk stabilization fixes and include date i18n formatting TODOs.
- **D-02:** Do not include settings auto-save debounce, countdown notification progress bars, or other deferred UX polish from backlog item 999.1.

### Error Feedback
- **D-03:** Add or improve UI feedback only for failures the user clearly needs to know about, especially user-triggered actions such as search, opening a comic, opening directories, batch export, and E-Hentai operations.
- **D-04:** Do not add duplicate notifications where an error already has UI feedback, and do not turn every internal `console.error` into a visible popup.

### TODO Boundary
- **D-05:** Include date i18n formatting TODOs in `ComicPane.vue`, `ComicCard.vue`, and `DownloadedComicCard.vue`.
- **D-06:** Leave `SearchPane.vue` sort-by-popularity support out of scope because it is a new capability, not stabilization cleanup.
- **D-07:** Leave the low-value Rust TODO in `download_format.rs` out of scope unless it naturally falls out of another change.

### Verification
- **D-08:** Use standard verification: run `pnpm build` and `cargo check`.
- **D-09:** Use code review to confirm key user paths do not produce duplicate or noisy notifications.

### the agent's Discretion
- The exact wording, placement, and severity of error messages can follow existing Naive UI `message` and `notification` patterns.
- Minor formatting, class cleanup, and import cleanup are allowed when directly tied to the scoped files.

</decisions>

<specifics>
## Specific Ideas

- Keep the phase conservative: fix polish and reliability issues without changing interaction models.
- Error feedback should help the user understand failed actions, not make background diagnostics noisy.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

No external specs or ADRs exist for this phase. Requirements are captured in this context and the active milestone entry in `.planning/ROADMAP.md`.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/AppContent.vue` uses Naive UI `useMessage` and `useNotification` for config save messages, log-size warnings, and E-Hentai auto-check feedback.
- `src/panes/SearchPane.vue` uses `useMessage` and `useNotification` for search warnings, invalid comic id, E-Hentai import validation, import failure, and import success.
- `src/components/SettingsDialog.vue` uses `useNotification` for E-Hentai connectivity validation and diagnostic results.
- `src/panes/DownloadedPane.vue` uses `useMessage` for batch export progress, success, warning, and errors.

### Established Patterns
- Frontend state lives in `src/store.ts` via Pinia refs.
- User-visible feedback is already built around Naive UI `message` for lightweight transient feedback and `notification` for richer errors or summaries.
- Backend command errors flow through `CommandResult` / `CommandError` and generated `src/bindings.ts` result wrappers.
- Existing frontend code often logs command failures with `console.error`; Phase 1 should convert only user-action failures that lack UI feedback.

### Integration Points
- Search and comic selection error paths: `src/panes/SearchPane.vue`.
- Downloaded/export error paths: `src/panes/DownloadedPane.vue`.
- App-level config, logs, locale, and startup E-Hentai auto-check paths: `src/AppContent.vue`.
- Date display TODOs: `src/panes/ComicPane.vue`, `src/components/ComicCard.vue`, `src/components/DownloadedComicCard.vue`.
- Backend verification: `src-tauri/src/commands.rs` and related Rust modules via `cargo check`.

</code_context>

<deferred>
## Deferred Ideas

- Countdown progress bar and auto-close behavior for popup notifications remains in backlog item 999.1.
- Debounced settings persistence for E-Hentai favorites limit changes remains in backlog item 999.1.
- Search sort by popularity remains out of scope for this phase and should become a future feature phase if desired.

</deferred>

---

*Phase: 01-stabilization-cleanup*
*Context gathered: 2026-05-29*
