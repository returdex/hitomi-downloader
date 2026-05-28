# Phase 1: Stabilization cleanup - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-05-29
**Phase:** 01-stabilization-cleanup
**Areas discussed:** Cleanup scope, error feedback, TODO boundary, verification

---

## Cleanup Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Conservative cleanup | Only clear low-risk issues such as user-visible error feedback, class/formatting defects, and build checks. | |
| Include small TODOs | Add date i18n formatting TODOs to the low-risk stabilization scope. | Yes |
| More aggressive cleanup | Also address settings auto-save noise, pulling backlog UX work forward. | |

**User's choice:** Include small TODOs.
**Notes:** Settings auto-save debounce and notification countdown behavior remain backlog work.

---

## Error Feedback

| Option | Description | Selected |
|--------|-------------|----------|
| User-visible failures only | Add UI feedback for user-triggered failures while avoiding duplicate or noisy popups. | Yes |
| All `console.error` paths | Convert every frontend `console.error` to visible UI feedback. | |
| Do not touch error feedback | Limit the phase to TODO and formatting cleanup. | |

**User's choice:** User-visible failures only.
**Notes:** Search, comic selection, directory opening, batch export, and E-Hentai operations are the main examples.

---

## TODO Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Date i18n only | Handle date formatting TODOs in user-visible cards/panes. | Yes |
| Date i18n plus Rust small TODO | Include a low-value Rust TODO if desired. | |
| All TODOs | Also include search sort by popularity. | |

**User's choice:** Date i18n only.
**Notes:** Search sort by popularity is treated as a new capability and deferred.

---

## Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Standard verification | Run `pnpm build`, `cargo check`, and review notification paths for duplicate noise. | Yes |
| Add manual verification | Also start the app or dev server and manually click key UI paths. | |
| Build checks only | Run build checks without extra notification review. | |

**User's choice:** Standard verification.
**Notes:** Manual app testing is not required for this phase unless implementation reveals unexpected UI risk.

---

## the agent's Discretion

- Exact wording and severity of user-facing feedback.
- Minor formatting/import cleanup directly tied to scoped files.

## Deferred Ideas

- Countdown progress bar and auto-close behavior for popup notifications.
- Debounced settings persistence for E-Hentai favorites limit changes.
- Search sort by popularity.
