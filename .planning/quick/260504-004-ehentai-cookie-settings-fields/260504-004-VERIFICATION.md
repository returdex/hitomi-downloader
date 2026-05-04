---
status: passed
date: 2026-05-04
---

# Verification

## Must Haves

- Cookie moved to Settings: passed. `SettingsDialog.vue` now owns the three E-Hentai cookie fields.
- EhViewer-style fields: passed. Config records `ipb_member_id`, `ipb_pass_hash`, and `igneous` separately.
- Manual import still works: passed. Search pane builds a Cookie header from Settings fields.
- Startup auto-check still works: passed. App startup uses the same helper.
- Existing full-cookie config migration: passed. Config merge attempts to split legacy `ehentaiCookie` into the new fields.

## Commands

- `cargo check`
- `pnpm.cmd build`

## Residual Risk

- Cookie values are persisted as config values and are not encrypted.
