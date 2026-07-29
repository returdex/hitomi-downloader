# Codebase Structure

**Analysis Date:** 2026-07-29

## Directory Layout

```text
hitomi/
├── .github/
│   ├── workflows/              # Cross-platform tagged-release pipeline
│   └── ISSUE_TEMPLATE/         # GitHub issue forms
├── .planning/
│   └── codebase/               # Generated codebase reference documents
├── dist/                       # Generated Vite production bundle
├── public/                     # Static frontend assets copied by Vite
├── src/                        # Vue/TypeScript WebView application
│   ├── assets/                 # Bundled frontend images
│   ├── components/             # Reusable dialogs, cards, controls, progress, logs
│   ├── locales/                # i18n setup and JSON dictionaries
│   ├── panes/                  # Top-level feature/workflow panes
│   ├── App.vue                 # Root Naive UI providers/theme
│   ├── AppContent.vue          # Main application layout and config lifecycle
│   ├── bindings.ts             # Generated Rust-to-TypeScript IPC contract
│   ├── main.ts                 # Vue bootstrap
│   ├── store.ts                # Shared Pinia state
│   ├── types.ts                # Frontend-only types
│   └── utils.ts                # Frontend helpers
├── src-tauri/                  # Rust/Tauri native application
│   ├── capabilities/           # Tauri permission capability documents
│   ├── icons/                  # Native package icons
│   ├── resources/              # Bundled runtime data
│   ├── src/
│   │   ├── hitomi/             # Hitomi search/index/URL domain implementation
│   │   ├── types/              # Native domain and IPC models
│   │   ├── commands.rs         # Tauri application command boundary
│   │   ├── config.rs           # Persistent user configuration
│   │   ├── download_manager.rs # Concurrent download state machine
│   │   ├── events.rs           # Typed native-to-frontend events
│   │   ├── export.rs           # PDF/CBZ archive production
│   │   ├── hitomi_client.rs    # HTTP gateway
│   │   ├── lib.rs              # Native composition root
│   │   ├── logger.rs           # Tracing and log persistence
│   │   └── main.rs             # Native binary entry
│   ├── Cargo.toml              # Rust dependencies and release profile
│   ├── tauri.conf.json         # Tauri product/build/bundle settings
│   └── build.rs                # Tauri build entry
├── index.html                  # Vite HTML entry
├── package.json                # Frontend scripts and dependency manifest
├── pnpm-lock.yaml              # Locked JavaScript dependency graph
├── tsconfig.json               # Frontend TypeScript policy
├── uno.config.ts               # UnoCSS configuration
└── vite.config.ts              # Frontend build and auto-import configuration
```

## Directory Purposes

**`src/`:**
- Purpose: Entire WebView-side user interface and reactive state
- Contains: Vue SFCs, TypeScript helpers/store, localization, generated bindings
- Key files: `src/main.ts`, `src/App.vue`, `src/AppContent.vue`, `src/store.ts`, `src/bindings.ts`

**`src/panes/`:**
- Purpose: Top-level user workflows placed in the main split-pane layout
- Contains: search, active download, downloaded library, and comic detail views
- Key files: `src/panes/SearchPane.vue`, `src/panes/DownloadingPane.vue`, `src/panes/DownloadedPane.vue`, `src/panes/ComicPane.vue`

**`src/components/`:**
- Purpose: Reusable UI units and modal content shared by panes/application shell
- Contains: comic cards, download button, settings/about dialogs, progress groups, log viewer, labeled inputs
- Key files: `src/components/ComicCard.vue`, `src/components/DownloadedComicCard.vue`, `src/components/SettingsDialog.vue`, `src/components/LogViewer.vue`

**`src/locales/`:**
- Purpose: Supported locale registry and translation dictionaries
- Contains: locale exports and parallel JSON message trees
- Key files: `src/locales/index.ts`, `src/locales/en-US.json`, `src/locales/zh-CN.json`

**`src-tauri/src/`:**
- Purpose: Native application composition, IPC adapters, domain services, and OS/network infrastructure
- Contains: Rust modules organized mostly by responsibility
- Key files: `src-tauri/src/lib.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/hitomi_client.rs`, `src-tauri/src/download_manager.rs`, `src-tauri/src/export.rs`

**`src-tauri/src/hitomi/`:**
- Purpose: Port Hitomi's index and image URL algorithms into Rust
- Contains: gallery DTOs, `gg.js` routing support, index/nozomi search, result-set composition
- Key files: `src-tauri/src/hitomi/mod.rs`, `src-tauri/src/hitomi/search.rs`, `src-tauri/src/hitomi/result.rs`, `src-tauri/src/hitomi/common.rs`, `src-tauri/src/hitomi/gg.rs`

**`src-tauri/src/types/`:**
- Purpose: Native domain types and serializable Rust/TypeScript contract models
- Contains: comic/search/config-adjacent enums, comic archive metadata
- Key files: `src-tauri/src/types/mod.rs`, `src-tauri/src/types/comic.rs`, `src-tauri/src/types/search_result.rs`, `src-tauri/src/types/comic_info.rs`

**`src-tauri/resources/`:**
- Purpose: Runtime data bundled into the native application
- Contains: Chinese E-Hentai tag translation dictionary
- Key files: `src-tauri/resources/ehtags-cn.json`

**`.github/workflows/`:**
- Purpose: Build platform installers and draft GitHub releases
- Contains: tag-triggered GitHub Actions workflow
- Key files: `.github/workflows/Publish.yml`

## Key File Locations

**Entry Points:**
- `index.html`: Vite host document and `src/main.ts` loader
- `src/main.ts`: Vue/Pinia/i18n initialization
- `src-tauri/src/main.rs`: native executable entry
- `src-tauri/src/lib.rs`: Tauri composition root and command/event registration
- `src-tauri/build.rs`: Cargo build-script entry

**Configuration:**
- `package.json`: JavaScript dependency and script authority
- `pnpm-lock.yaml`: exact frontend dependency resolution
- `tsconfig.json`: strict browser TypeScript rules
- `vite.config.ts`: Vue/JSX/UnoCSS/auto-import plugins and Tauri dev server
- `uno.config.ts`: styling presets and transformers
- `eslint.config.js`: JS/TS/Vue flat lint configuration
- `.prettierrc`: source formatting policy
- `src-tauri/Cargo.toml`: Rust dependencies and release optimization
- `src-tauri/Cargo.lock`: exact native dependency resolution
- `src-tauri/tauri.conf.json`: product identity, frontend commands, window, and packaging
- `src-tauri/capabilities/default.json`: main-window Tauri permissions

**Core Logic:**
- `src-tauri/src/commands.rs`: all frontend-callable application operations
- `src-tauri/src/hitomi_client.rs`: external HTTP client construction and gateway
- `src-tauri/src/hitomi/`: search and remote gallery algorithms
- `src-tauri/src/download_manager.rs`: download concurrency/task lifecycle
- `src-tauri/src/export.rs`: PDF/CBZ writing
- `src-tauri/src/types/comic.rs`: central comic domain model and metadata conversion
- `src-tauri/src/config.rs`: persistent settings/defaults/migrations

**Testing:**
- `src-tauri/src/translator.rs`: co-located Rust unit tests
- No `tests/`, frontend `*.test.*`, frontend `*.spec.*`, or dedicated test configuration is present

## Naming Conventions

**Files:**
- Vue components and panes use PascalCase: `src/components/DownloadButton.vue`, `src/panes/SearchPane.vue`
- Frontend TypeScript modules use lowercase descriptive names: `src/store.ts`, `src/types.ts`, `src/utils.ts`
- Rust modules use snake_case: `src-tauri/src/download_manager.rs`, `src-tauri/src/hitomi_client.rs`
- Rust module directories expose a `mod.rs`: `src-tauri/src/hitomi/mod.rs`, `src-tauri/src/types/mod.rs`
- Locale dictionaries use locale tags: `src/locales/en-US.json`, `src/locales/zh-CN.json`

**Directories:**
- Frontend groups by UI role: `src/panes/`, `src/components/`, `src/locales/`
- Native code groups complex domains into singular lowercase modules: `src-tauri/src/hitomi/`, `src-tauri/src/types/`
- Platform configuration stays under `src-tauri/`; do not mix native files into `src/`

## Where to Add New Code

**New Frontend Feature:**
- Primary workflow/view: `src/panes/<Feature>Pane.vue` when it occupies a main application pane
- Reusable UI: `src/components/<Feature>.vue`
- Shared state: extend `src/store.ts` only when multiple panes/components own the same live state
- Frontend-only types/helpers: `src/types.ts` or `src/utils.ts`
- Translations: add matching keys to both `src/locales/en-US.json` and `src/locales/zh-CN.json`
- Layout/navigation wiring: `src/AppContent.vue`
- Tests: no established frontend location; introduce a test tool and documented `src/**/*.test.ts` convention before adding isolated frontend tests

**New Native Capability:**
- IPC adapter: add a focused `#[tauri::command]` in `src-tauri/src/commands.rs`
- Command registration: include it in `collect_commands!` in `src-tauri/src/lib.rs`
- Domain implementation: create a focused snake_case module in `src-tauri/src/`, or add it beneath `src-tauri/src/hitomi/` if it is Hitomi-specific
- Shared serializable model: add a file under `src-tauri/src/types/`, export it from `src-tauri/src/types/mod.rs`, and derive Serde/Specta traits as appropriate
- Long-running UI feedback: define a typed event in `src-tauri/src/events.rs`, register it in `src-tauri/src/lib.rs`, and listen through regenerated `src/bindings.ts`
- Generated frontend contract: never edit `src/bindings.ts`; regenerate it by running the debug Tauri application

**New External Integration:**
- HTTP transport/client policy: `src-tauri/src/hitomi_client.rs` for related Hitomi/E-Hentai access; use a new focused native module for an unrelated service
- Input validation and IPC mapping: `src-tauri/src/commands.rs`
- User-configurable integration values: `src-tauri/src/config.rs` plus UI in `src/components/SettingsDialog.vue`
- Tauri capability changes: `src-tauri/capabilities/default.json`

**New Download or Export Format:**
- Format enum/model: `src-tauri/src/types/download_format.rs` or a new type under `src-tauri/src/types/`
- Download conversion and persistence: `src-tauri/src/download_manager.rs`
- Archive implementation: `src-tauri/src/export.rs`
- Command/event exposure: `src-tauri/src/commands.rs`, `src-tauri/src/events.rs`, and registration in `src-tauri/src/lib.rs`
- UI action/status: `src/components/DownloadButton.vue` or `src/panes/DownloadedPane.vue`

**Utilities:**
- Frontend shared helpers: `src/utils.ts`
- Small native cross-module helpers: `src-tauri/src/utils.rs`
- Error-chain extensions: `src-tauri/src/extensions.rs`
- Keep business rules in their owning native domain module rather than accumulating them in utility files

## Special Directories

**`dist/`:**
- Purpose: Vite production output consumed by Tauri packaging
- Generated: Yes
- Committed: No, excluded by `.gitignore`; regenerate with `pnpm build`

**`node_modules/`:**
- Purpose: Installed frontend dependencies
- Generated: Yes
- Committed: No, excluded by `.gitignore`

**`src-tauri/target/`:**
- Purpose: Cargo/Tauri native build output and platform bundles
- Generated: Yes
- Committed: No, excluded by `.gitignore`

**`src-tauri/icons/`:**
- Purpose: Source icons for platform installers/application metadata
- Generated: No
- Committed: Yes

**`src-tauri/resources/`:**
- Purpose: Data embedded or bundled with the desktop application
- Generated: No
- Committed: Yes

**`.planning/codebase/`:**
- Purpose: GSD reference material used for future implementation planning
- Generated: Yes
- Committed: Managed by the surrounding GSD workflow

**`auto-imports.d.ts` and `components.d.ts`:**
- Purpose: Generated declarations for auto-imported Vue/Naive UI APIs and components
- Generated: Yes
- Committed: Present; update through the Vite plugins configured in `vite.config.ts`

---

*Structure analysis: 2026-07-29*
