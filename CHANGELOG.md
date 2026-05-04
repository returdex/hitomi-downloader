# Changelog

## v0.1.1 - 2026-05-05

### Added

- E-Hentai favorites import from saved login cookies.
- EhViewer-style E-Hentai cookie fields in Settings: `ipb_member_id`, `ipb_pass_hash`, and `igneous`.
- Startup auto-check for new E-Hentai favorites.
- Per-import download folder selection for favorites downloads.
- E-Hentai connectivity diagnostics from Settings.
- Batch export for local comics to PDF or CBZ.
- Optional skip for existing exported PDF/CBZ files.
- Chinese tag translation for CBZ `ComicInfo.xml`.

### Fixed

- E-Hentai favorites requests now use the configured app proxy mode.
- E-Hentai requests use browser-like headers.
- E-Hentai `451` responses with parsable HTML are accepted instead of failing immediately.
- Removed leftover debug logging from the comic details pane.
- Fixed a malformed CSS class in the comic details tag buttons.
