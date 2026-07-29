# Hitomi Downloader Project

## Purpose

Hitomi Downloader is a Tauri desktop app for searching, downloading, browsing, and exporting hitomi.la galleries.

## Stack

- Vue 3 and TypeScript frontend
- Naive UI, Pinia, UnoCSS, and vue-i18n
- Tauri v2 with Rust backend commands

## Working Notes

- Keep quick fixes narrow and verify with the existing build pipeline when possible.
- Quick tasks are tracked under `.planning/quick/`.

## Current State

v1.0 Stabilization shipped on 2026-07-29. The release includes localized comic
dates, focused user-action error feedback, E-Hentai favorites workflows,
transactional pagination state, automated frontend tests, and complete Vue/Rust
validation evidence. The formal release candidate is `5427e99`.

Validated requirements: PHASE-1, REL-01, UI-01, and VAL-01.

Known non-blocking debt includes existing Vite chunk-size and Rust dead-code
warnings, plus plain-text storage of E-Hentai cookie fields. The settings-save
debounce remains preserved as uncommitted work for the next milestone.

## Next Milestone Goals

- Decide whether to ship the preserved settings-save debounce and notification
  polish from Backlog Phase 999.1.
- Continue improving E-Hentai credential storage security.
- Refresh codebase mapping for the newly documented repository paths when the
  next planning cycle begins.

---
*Last updated: 2026-07-29 after v1.0 milestone*
