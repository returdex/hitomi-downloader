---
status: passed
date: 2026-05-04
---

# Verification

## Must Haves

- Cookie record: passed. `Config` includes saved E-Hentai cookie and import settings.
- Startup update check: passed. `AppContent.vue` runs an automatic check when the saved auto-check flag is enabled.
- Avoid repeated downloads: passed. Known gallery ids are saved and sent to the backend so startup checks queue only new favorites.
- Existing manual flow still works: passed. Search pane import still uses the same command and updates known ids after success.

## Commands

- `cargo check`
- `pnpm.cmd build`

## Residual Risk

- Cookie storage is plain config storage, not OS keychain encryption.
- Automatic checking is limited to the configured favorites page and optional limit.
