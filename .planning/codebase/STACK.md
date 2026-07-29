# Technology Stack

**Analysis Date:** 2026-07-29

## Languages

**Primary:**
- Rust 2021 edition - native application runtime, networking, downloads, exports, configuration, and logging under `src-tauri/src/`
- TypeScript 5.6 - Vue application bootstrap, shared state, generated Tauri bindings, and frontend utilities under `src/`

**Secondary:**
- Vue Single-File Components - UI composition and pane/component templates in `src/**/*.vue`
- JSON - localization dictionaries in `src/locales/*.json`, Tauri configuration in `src-tauri/tauri.conf.json`, and bundled tag data in `src-tauri/resources/ehtags-cn.json`
- CSS via UnoCSS utilities - component styling in `src/**/*.vue`, configured by `uno.config.ts`
- HTML - Vite host document in `index.html`

## Runtime

**Environment:**
- Tauri 2 native desktop runtime with the platform WebView; application bootstrapping is in `src-tauri/src/main.rs` and `src-tauri/src/lib.rs`
- Vue 3.5 browser runtime inside the Tauri WebView; frontend entry is `src/main.ts`
- Tokio 1.45 async runtime through Tauri for network and download work in `src-tauri/src/download_manager.rs`, `src-tauri/src/hitomi_client.rs`, and `src-tauri/src/hitomi/`
- Node.js LTS is used for frontend builds in `.github/workflows/Publish.yml`; no exact local Node version file is present
- Rust stable is used by CI in `.github/workflows/Publish.yml`; no repository-pinned toolchain file is present

**Package Manager:**
- pnpm 9.5.0, declared by `package.json`
- Lockfile: present at `pnpm-lock.yaml`
- Cargo, using `src-tauri/Cargo.toml`
- Rust lockfile: present at `src-tauri/Cargo.lock`

## Frameworks

**Core:**
- Tauri 2 - native application shell, application state, IPC commands, events, dialogs, and filesystem opener integration in `src-tauri/src/lib.rs`
- Vue 3.5.13 - component-based frontend using `<script setup>` in `src/**/*.vue`
- Pinia 3.0.2 - global frontend state in `src/store.ts`
- Naive UI 2.41.0 - UI controls and provider infrastructure in `src/App.vue`, panes, and components
- Vue I18n 11 - runtime locale selection and dictionaries initialized in `src/main.ts`
- UnoCSS 66.1.3 - utility and attributify styling configured in `uno.config.ts`

**Testing:**
- Rust built-in test harness - unit tests are embedded in modules such as `src-tauri/src/translator.rs`
- No frontend test runner or test command is declared in `package.json`

**Build/Dev:**
- Vite 6.0.3 - frontend dev server and production bundle, configured in `vite.config.ts`
- Vue TSC 2.1.10 - strict frontend type checking through the `build` script in `package.json`
- TypeScript 5.6.2 - ES2020 target and bundler resolution in `tsconfig.json`
- Tauri CLI 2 - desktop development and packaging through `pnpm tauri`, declared in `package.json`
- tauri-build 2 - native build script integration in `src-tauri/build.rs`
- ESLint 9 with TypeScript ESLint 8.33 and eslint-plugin-vue 10.1 - lint configuration in `eslint.config.js`; no lint script is declared
- Prettier 3.5.3 - formatting configuration in `.prettierrc`
- unplugin-auto-import 19.3 and unplugin-vue-components 28.7 - generated Vue/Naive UI imports and component declarations via `vite.config.ts`, `auto-imports.d.ts`, and `components.d.ts`

## Key Dependencies

**Critical:**
- `reqwest` 0.12.19 - HTTP transport for Hitomi, image CDN, and E-Hentai requests in `src-tauri/src/hitomi_client.rs`
- `reqwest-middleware` 0.4.2 and `reqwest-retry` 0.7.0 - separate API/image clients with retry policies in `src-tauri/src/hitomi_client.rs`
- `tauri-specta` 2.0.0-rc.20, `specta` 2.0.0-rc.20, and `specta-typescript` 0.0.7 - type-safe Rust command/event bindings generated into `src/bindings.ts` from `src-tauri/src/lib.rs`
- `tokio` 1.45.1 - task spawning, semaphores, watch channels, and join sets in `src-tauri/src/download_manager.rs`
- `parking_lot` 0.12.4 - shared configuration, HTTP clients, and task-map locks in `src-tauri/src/config.rs`, `src-tauri/src/hitomi_client.rs`, and `src-tauri/src/download_manager.rs`
- `image` 0.25.6 - downloaded image conversion to WebP in `src-tauri/src/download_manager.rs`
- Git-sourced `lopdf` with `embed_image_webp` - PDF assembly in `src-tauri/src/export.rs`
- `zip` 4.0.0 and `yaserde` 0.12.0 - CBZ archive and `ComicInfo.xml` production in `src-tauri/src/export.rs` and `src-tauri/src/types/comic_info.rs`

**Infrastructure:**
- `tracing`, `tracing-subscriber`, and `tracing-appender` - console, frontend-event, and rolling-file logs in `src-tauri/src/logger.rs`
- `notify` 8.0.0 - watches the log directory and recreates logging after removal in `src-tauri/src/logger.rs`
- `serde` and `serde_json` - IPC models, config persistence, metadata, and API parsing across `src-tauri/src/`
- `futures` 0.3.31 - concurrent gallery and search result aggregation in `src-tauri/src/hitomi_client.rs` and `src-tauri/src/hitomi/result.rs`
- `indexmap` 2.9.0 - stable-order result sets and downloaded-comic deduplication in `src-tauri/src/hitomi/` and `src-tauri/src/commands.rs`
- `walkdir` 2.5.0 - recursive local-library discovery in `src-tauri/src/commands.rs`
- `sha2`, `byteorder`, and `regex-lite` - Hitomi index lookup and parsing in `src-tauri/src/hitomi/search.rs`
- `@tauri-apps/plugin-dialog` and `tauri-plugin-dialog` 2 - native path selection exposed to frontend components
- `@tauri-apps/plugin-opener` and `tauri-plugin-opener` 2 - reveal local files/directories from `src-tauri/src/commands.rs`

## Configuration

**Environment:**
- No `.env` file is present at repository root, and application runtime does not depend on environment-variable secrets
- `TAURI_DEV_HOST` optionally changes Vite host/HMR behavior in `vite.config.ts`
- User settings are persisted as `config.json` under Tauri's application-data directory by `src-tauri/src/config.rs`
- Settings include download/export directories, locale, file logging, image format, directory template, proxy mode/host/port, and E-Hentai session fields in `src-tauri/src/config.rs`

**Build:**
- Frontend: `package.json`, `vite.config.ts`, `tsconfig.json`, `tsconfig.node.json`, `uno.config.ts`, `eslint.config.js`, and `.prettierrc`
- Native: `src-tauri/Cargo.toml`, `src-tauri/build.rs`, `src-tauri/tauri.conf.json`, and `src-tauri/capabilities/default.json`
- Tauri runs `pnpm dev` on port 5005 for development and `pnpm build` before packaging, as specified in `src-tauri/tauri.conf.json`
- Production frontend output is `dist/`, consumed through `frontendDist` in `src-tauri/tauri.conf.json`

## Platform Requirements

**Development:**
- Install Node.js, pnpm 9-compatible tooling, Rust stable, and Tauri 2 platform prerequisites; setup commands are documented in `README.md`
- On Linux, WebKitGTK 4.1, AppIndicator, librsvg, and patchelf are installed by `.github/workflows/Publish.yml`
- Use `pnpm install`, then `pnpm tauri dev`; use `pnpm tauri build` for native packages as documented in `README.md`

**Production:**
- Packaged desktop targets are Windows x64, Linux amd64, macOS aarch64, and macOS x86_64 through `.github/workflows/Publish.yml`
- Windows produces NSIS and portable artifacts, Linux produces DEB/RPM/portable archives, and macOS produces DMG files
- Release builds enable stripping, LTO, one codegen unit, and abort-on-panic in `src-tauri/Cargo.toml`

---

*Stack analysis: 2026-07-29*
