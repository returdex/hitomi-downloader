# External Integrations

**Analysis Date:** 2026-07-29

## APIs & External Services

**Comic Metadata and Search:**
- Hitomi (`hitomi.la`) - searches index data, resolves gallery metadata, retrieves suggestions, and derives image URLs
  - SDK/Client: custom Rust implementation in `src-tauri/src/hitomi/` over `reqwest` clients from `src-tauri/src/hitomi_client.rs`
  - Endpoints: gallery scripts and index/nozomi data are assembled from constants and algorithms in `src-tauri/src/hitomi/mod.rs`, `src-tauri/src/hitomi/search.rs`, and `src-tauri/src/hitomi/common.rs`
  - Auth: none

**Image Delivery:**
- Hitomi-generated content CDN (`gold-usergeneratedcontent.net`) - gallery images, cover data, and the `gg.js` URL-routing rules
  - SDK/Client: custom `reqwest` image and cover clients in `src-tauri/src/hitomi_client.rs`
  - URL construction: `src-tauri/src/hitomi/gg.rs` and `src-tauri/src/hitomi/common.rs`
  - Auth: none; requests set a `https://hitomi.la/` referrer

**Favorites Import:**
- E-Hentai / ExHentai favorites pages - imports gallery IDs from a user's saved favorites and provides connectivity diagnostics
  - SDK/Client: custom HTML fetch and link parsing in `src-tauri/src/hitomi_client.rs` and `src-tauri/src/commands.rs`
  - Auth: E-Hentai cookie components stored in application config fields defined in `src-tauri/src/config.rs`, not environment variables
  - URL policy: commands accept only `https://e-hentai.org/` or `https://exhentai.org/` prefixes in `src-tauri/src/commands.rs`

**Desktop Operating System:**
- Tauri dialog plugin - native file/directory selection used by frontend settings and export flows
  - SDK/Client: `@tauri-apps/plugin-dialog` in `package.json` and `tauri-plugin-dialog` in `src-tauri/Cargo.toml`
  - Permission: `dialog:default` in `src-tauri/capabilities/default.json`
- Tauri opener plugin - reveals downloaded/exported items in the platform file manager
  - SDK/Client: `@tauri-apps/plugin-opener` and `tauri-plugin-opener`
  - Implementation: `show_path_in_file_manager` in `src-tauri/src/commands.rs`
  - Permission: `opener:default` in `src-tauri/capabilities/default.json`

## Data Storage

**Databases:**
- Not detected; there is no SQL, ORM, embedded database, or remote persistence layer in `package.json` or `src-tauri/Cargo.toml`

**File Storage:**
- Local filesystem only
- Application configuration: Tauri app-data `config.json`, read, migrated, and written by `src-tauri/src/config.rs`
- Downloaded gallery: configurable directory containing images plus `metadata.json`, created and read by `src-tauri/src/download_manager.rs`, `src-tauri/src/types/comic.rs`, and `src-tauri/src/commands.rs`
- Exports: configurable directory containing PDF and CBZ archives produced by `src-tauri/src/export.rs`
- Logs: daily `hitomi-downloader*.log` files beneath the Tauri app-data `logs/` directory from `src-tauri/src/logger.rs`
- Bundled translation data: `src-tauri/resources/ehtags-cn.json`, loaded by `src-tauri/src/translator.rs`

**Caching:**
- No external cache
- In-memory frontend cover blob URLs and search/download state are held by Pinia in `src/store.ts`
- Hitomi index versions use Tokio `OnceCell` process-level caches in `src-tauri/src/hitomi/search.rs`
- Active native download tasks are held in an in-memory map in `src-tauri/src/download_manager.rs`

## Authentication & Identity

**Auth Provider:**
- No application account or identity provider
- Optional E-Hentai session authentication is custom:
  - UI stores `ipb_member_id`, `ipb_pass_hash`, and `igneous` values in application configuration defined by `src-tauri/src/config.rs`
  - Frontend assembles the HTTP cookie through `buildEhentaiCookie` in `src/utils.ts`
  - Backend passes the cookie only to validated E-Hentai/ExHentai requests in `src-tauri/src/hitomi_client.rs` and `src-tauri/src/commands.rs`

## Monitoring & Observability

**Error Tracking:**
- None; no Sentry or remote telemetry SDK is declared in `package.json` or `src-tauri/Cargo.toml`

**Logs:**
- Rust `tracing` emits to stdout, optional daily rolling local files, and typed Tauri `LogEvent` messages in `src-tauri/src/logger.rs`
- Frontend displays live native log events in `src/components/LogViewer.vue`
- The application warns when the local logs directory exceeds 50 MiB in `src/AppContent.vue`
- Errors crossing Tauri IPC use structured `CommandError` values from `src-tauri/src/errors.rs`

## CI/CD & Deployment

**Hosting:**
- GitHub Releases hosts packaged desktop artifacts created by `.github/workflows/Publish.yml`
- There is no hosted web application or backend service

**CI Pipeline:**
- GitHub Actions workflow `.github/workflows/Publish.yml` triggers on `v*` tags
- It builds Windows, Linux, and both macOS architectures with Node LTS, pnpm, Rust stable, and `tauri-apps/tauri-action`
- A final job uploads artifacts into a draft GitHub Release with the workflow-scoped GitHub token
- No continuous test or lint workflow is present under `.github/workflows/`

## Environment Configuration

**Required env vars:**
- None for local runtime
- `TAURI_DEV_HOST` is optional for remote-device/HMR development and is read by `vite.config.ts`
- CI obtains `GITHUB_TOKEN` automatically for release creation in `.github/workflows/Publish.yml`

**Secrets location:**
- E-Hentai session fields are stored in Tauri's local app-data `config.json` by `src-tauri/src/config.rs`
- GitHub release credentials use GitHub Actions' built-in secret context in `.github/workflows/Publish.yml`
- No repository-root `.env` file is present

## Webhooks & Callbacks

**Incoming:**
- None; this is a local desktop application with no HTTP server
- Frontend-to-native requests use Tauri IPC commands generated in `src/bindings.ts` and registered in `src-tauri/src/lib.rs`

**Outgoing:**
- No webhooks
- Native-to-frontend callbacks use typed Tauri events: log, download-task, download-speed, PDF-export, and CBZ-export models in `src-tauri/src/events.rs`
- Event listeners are implemented in `src/components/LogViewer.vue`, `src/panes/DownloadingPane.vue`, and `src/panes/DownloadedPane.vue`

---

*Integration audit: 2026-07-29*
