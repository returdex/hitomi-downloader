<!-- refreshed: 2026-07-29 -->
# Architecture

**Analysis Date:** 2026-07-29

## System Overview

```text
┌─────────────────────────────────────────────────────────────────────┐
│                    Vue Presentation Layer                           │
├─────────────────────┬──────────────────────┬────────────────────────┤
│ App shell/providers │ Feature panes        │ Reusable components    │
│ `src/App*.vue`      │ `src/panes/`         │ `src/components/`      │
└──────────┬──────────┴──────────┬───────────┴────────────┬───────────┘
           │                     │                        │
           └──────────────┬──────┴────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Frontend State and Typed IPC                                       │
│ `src/store.ts` · `src/types.ts` · `src/bindings.ts`                │
└─────────────────────────┬───────────────────────────────────────────┘
                          │ Tauri commands / typed events
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Native Application / Command Boundary                              │
│ `src-tauri/src/lib.rs` · `src-tauri/src/commands.rs`                │
└─────────────┬────────────────────┬──────────────────────┬───────────┘
              ▼                    ▼                      ▼
┌────────────────────┐ ┌──────────────────────┐ ┌─────────────────────┐
│ Hitomi HTTP/domain │ │ Download and export  │ │ Config and logging  │
│ `hitomi_client.rs` │ │ `download_manager.rs`│ │ `config.rs`         │
│ `hitomi/`          │ │ `export.rs`          │ │ `logger.rs`         │
└─────────┬──────────┘ └──────────┬───────────┘ └──────────┬──────────┘
          ▼                       ▼                        ▼
┌─────────────────────────────────────────────────────────────────────┐
│ External HTTP services and local filesystem                         │
│ Hitomi/E-Hentai · downloads/metadata · PDF/CBZ · config/log files  │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| Frontend bootstrap | Creates Vue, Pinia, and i18n instances and mounts the root component | `src/main.ts` |
| Application shell | Provides Naive UI context, global navigation, configuration lifecycle, and automatic favorites check | `src/App.vue`, `src/AppContent.vue` |
| Feature panes | Own search, active download, downloaded-library, and comic-detail workflows | `src/panes/SearchPane.vue`, `src/panes/DownloadingPane.vue`, `src/panes/DownloadedPane.vue`, `src/panes/ComicPane.vue` |
| Global frontend store | Shares config, selected comic/tab, progress maps, covers, and search results | `src/store.ts` |
| Typed bridge | Generated command functions, result types, event models, and listeners | `src/bindings.ts` |
| Tauri composition root | Registers commands/events/plugins and manages process-wide services | `src-tauri/src/lib.rs` |
| Command adapters | Validates IPC input, invokes domain services, maps failures to `CommandError`, and returns serializable data | `src-tauri/src/commands.rs` |
| HTTP gateway | Owns API/image/cover clients, retries, timeouts, proxy selection, and E-Hentai fetches | `src-tauri/src/hitomi_client.rs` |
| Hitomi domain | Implements index search, gallery parsing, URL derivation, and suggestions | `src-tauri/src/hitomi/` |
| Download manager | Owns task state, concurrency limits, pause/resume/cancel control, image conversion, metadata persistence, and progress events | `src-tauri/src/download_manager.rs` |
| Archive exporter | Writes PDF/CBZ output and emits export lifecycle events | `src-tauri/src/export.rs` |
| Persistent configuration | Defines defaults, loads/migrates `config.json`, and saves settings | `src-tauri/src/config.rs` |
| Domain models | Converts Hitomi gallery data into app comics and local metadata | `src-tauri/src/types/` |
| Observability | Configures console/file/frontend tracing and log-directory watching | `src-tauri/src/logger.rs` |

## Pattern Overview

**Overall:** Layered desktop architecture with a Vue MVVM-style frontend, a typed Tauri IPC boundary, and stateful Rust application services.

**Key Characteristics:**
- Keep UI rendering and interaction in Vue; invoke native capabilities through generated `commands`/`events` in `src/bindings.ts`.
- Treat `src-tauri/src/commands.rs` as an application-service boundary, not as the source of frontend state.
- Manage long-lived native dependencies through Tauri state initialized in `src-tauri/src/lib.rs`.
- Use typed Tauri events for ongoing operations that cannot be represented by one request/response, including downloads, exports, and logs.
- Persist durable user/library state to files; keep current UI selections and active tasks in memory.

## Layers

**Presentation Layer:**
- Purpose: Render application workflows and collect user input
- Location: `src/App.vue`, `src/AppContent.vue`, `src/panes/`, `src/components/`
- Contains: Vue SFCs, TSX notification content, Naive UI components, UnoCSS classes
- Depends on: `src/store.ts`, `src/utils.ts`, `src/bindings.ts`, `src/locales/`
- Used by: Tauri WebView through `src/main.ts`

**Frontend State/Bridge Layer:**
- Purpose: Coordinate shared reactive state and isolate the UI from raw Tauri invocation/event APIs
- Location: `src/store.ts`, `src/types.ts`, `src/bindings.ts`
- Contains: Pinia refs/actions, UI-only types, generated Rust model/command/event definitions
- Depends on: Vue, Pinia, `@tauri-apps/api`
- Used by: panes and components under `src/`

**IPC/Application Layer:**
- Purpose: Expose native operations and convert native failures to stable serializable results
- Location: `src-tauri/src/lib.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/errors.rs`, `src-tauri/src/events.rs`
- Contains: Tauri command handlers, event schemas, setup/registration, error DTOs
- Depends on: application services and domain models
- Used by: generated `src/bindings.ts`

**Domain/Service Layer:**
- Purpose: Search and materialize comics, manage downloads, construct archives, and translate tags
- Location: `src-tauri/src/hitomi/`, `src-tauri/src/hitomi_client.rs`, `src-tauri/src/download_manager.rs`, `src-tauri/src/export.rs`, `src-tauri/src/translator.rs`, `src-tauri/src/types/`
- Contains: HTTP algorithms, task state machines, file processing, serializable domain types
- Depends on: Tokio, reqwest, Tauri `AppHandle`, filesystem libraries
- Used by: `src-tauri/src/commands.rs`

**Infrastructure Layer:**
- Purpose: Persist configuration, write logs, access OS paths, and connect to external HTTP services
- Location: `src-tauri/src/config.rs`, `src-tauri/src/logger.rs`, `src-tauri/src/utils.rs`, `src-tauri/src/hitomi_client.rs`
- Contains: filesystem adapters, tracing configuration, HTTP clients, globally available app handle
- Depends on: Tauri runtime and local OS
- Used by: native services throughout `src-tauri/src/`

## Data Flow

### Primary Search Request Path

1. `SearchPane` collects a query and calls the generated command (`src/panes/SearchPane.vue`, `src/bindings.ts:20`)
2. Tauri dispatches to `search`, which delegates to managed `HitomiClient` (`src-tauri/src/commands.rs:128`)
3. `HitomiClient::search` resolves matching IDs through the Hitomi index and fetches a 25-item page (`src-tauri/src/hitomi_client.rs:79`)
4. Search algorithms combine positive/negative term result sets (`src-tauri/src/hitomi/result.rs:20`)
5. Gallery scripts become `Comic`/`SearchResult` models (`src-tauri/src/hitomi/common.rs:258`, `src-tauri/src/types/search_result.rs`)
6. Structured results cross IPC and are stored/rendered by the frontend (`src/store.ts`, `src/panes/SearchPane.vue`)

### Download Task Flow

1. A frontend comic action invokes `createDownloadTask` through `src/bindings.ts:44`
2. `create_download_task` delegates to managed `DownloadManager` (`src-tauri/src/commands.rs:171`)
3. `DownloadManager` inserts an in-memory task and spawns async work, with two concurrent comics and four concurrent images (`src-tauri/src/download_manager.rs:60`)
4. Image URLs are derived by `src-tauri/src/hitomi/common.rs`, bytes are fetched by `src-tauri/src/hitomi_client.rs`, and converted files/metadata are written by `src-tauri/src/download_manager.rs`
5. Create/update and aggregate speed events are emitted from `src-tauri/src/download_manager.rs` using schemas in `src-tauri/src/events.rs`
6. `DownloadingPane` listens and updates Pinia progress state (`src/panes/DownloadingPane.vue`, `src/store.ts`)

### Configuration Flow

1. Tauri setup creates/loads `Config`, ensures the application-data directory, and manages an `RwLock<Config>` (`src-tauri/src/lib.rs:79`)
2. Frontend loads config after mount and places it in Pinia (`src/AppContent.vue`, `src/store.ts`)
3. Deep reactive changes are debounced for one second and sent through `saveConfig` (`src/AppContent.vue`)
4. Backend persists pretty JSON and reloads HTTP clients/file logging when relevant fields change (`src-tauri/src/commands.rs:80`, `src-tauri/src/config.rs`)

### Local Library and Export Flow

1. `get_downloaded_comics` recursively finds `metadata.json`, sorts by modification time, reconstructs `Comic` values, and deduplicates by ID (`src-tauri/src/commands.rs:460`)
2. `DownloadedPane` requests PDF/CBZ exports (`src/panes/DownloadedPane.vue`, `src/bindings.ts`)
3. `src-tauri/src/export.rs` creates archives in the configured export directory and emits start/end/error events
4. `DownloadedPane` listens to typed export events and updates operation state (`src/panes/DownloadedPane.vue`)

**State Management:**
- Frontend shared state is a single setup-style Pinia store in `src/store.ts`; component-local dialog/timer state remains in Vue refs.
- Native application state is managed by Tauri: locked `Config`, cloneable `HitomiClient`, and cloneable `DownloadManager` are installed in `src-tauri/src/lib.rs`.
- Active download tasks are process-local and do not survive application restart; completed library state is reconstructed from per-gallery `metadata.json`.

## Key Abstractions

**Generated IPC Contract:**
- Purpose: Keep frontend calls and events aligned with Rust definitions
- Examples: `src-tauri/src/lib.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/events.rs`, `src/bindings.ts`
- Pattern: `tauri-specta` derives/collects Rust APIs and regenerates TypeScript bindings in debug builds

**Comic:**
- Purpose: Canonical app representation of remote gallery information plus local download location
- Examples: `src-tauri/src/types/comic.rs`, generated counterpart in `src/bindings.ts`
- Pattern: domain object with constructors/conversion from remote gallery data and local metadata

**DownloadManager:**
- Purpose: Own concurrent task lifecycle and frontend progress notification
- Examples: `src-tauri/src/download_manager.rs`, managed at `src-tauri/src/lib.rs:100`
- Pattern: cloneable service with `Arc`, locks, atomics, Tokio semaphores, watch channels, and event emission

**HitomiClient:**
- Purpose: Centralize proxy-aware/retrying HTTP access and high-level comic operations
- Examples: `src-tauri/src/hitomi_client.rs`, managed at `src-tauri/src/lib.rs:97`
- Pattern: application-scoped gateway containing separately tuned API, image, and cover clients

**CommandResult / CommandError:**
- Purpose: Return serializable error chains without exposing Rust errors over IPC
- Examples: `src-tauri/src/errors.rs`, `src-tauri/src/commands.rs`
- Pattern: command adapter maps contextual `anyhow` errors to a stable DTO

## Entry Points

**WebView Entry:**
- Location: `index.html` and `src/main.ts`
- Triggers: Vite loads the application into Tauri's WebView
- Responsibilities: create Vue app, install Pinia/i18n, mount `src/App.vue`

**Native Binary Entry:**
- Location: `src-tauri/src/main.rs`
- Triggers: packaged desktop executable or `pnpm tauri dev`
- Responsibilities: call `hitomi_downloader_lib::run()`

**Native Composition Root:**
- Location: `src-tauri/src/lib.rs`
- Triggers: native binary startup
- Responsibilities: register typed commands/events and plugins, create app data, initialize managed services and logging, run Tauri

**Build Entry:**
- Location: `src-tauri/build.rs`
- Triggers: Cargo/Tauri build
- Responsibilities: run Tauri build-time code generation

## Architectural Constraints

- **Threading:** The WebView frontend runs on the browser event loop. Native I/O runs on Tokio; download concurrency is explicitly limited to two comics and four images in `src-tauri/src/download_manager.rs`.
- **Global state:** Tauri-managed application state lives in `src-tauri/src/lib.rs`; a process-global `AppHandle` `OnceLock` exists in `src-tauri/src/utils.rs`; logger reload functions/guards are process globals in `src-tauri/src/logger.rs`; index versions use `OnceCell` in `src-tauri/src/hitomi/search.rs`.
- **Circular imports:** No explicit circular Rust module chain is detected. Frontend components should continue depending downward on `src/store.ts`/`src/bindings.ts`, not import other feature panes for shared behavior.
- **Persistence:** There is no database. Any durable feature must fit the config/per-gallery metadata/export filesystem model or introduce an explicit new storage adapter.
- **IPC compatibility:** Rust models exposed through commands/events must remain serializable and Specta-compatible; regenerate `src/bindings.ts` through the debug Tauri build rather than editing it.
- **Network boundary:** Hitomi and E-Hentai requests belong in `src-tauri/src/hitomi_client.rs` or `src-tauri/src/hitomi/`; the Vue layer does not perform remote HTTP calls directly.

## Anti-Patterns

### Editing Generated Bindings

**What happens:** A frontend change is made directly in `src/bindings.ts`.
**Why it's wrong:** `src/bindings.ts` is overwritten by `tauri-specta` during debug startup and will diverge from Rust.
**Do this instead:** Change command/event/model definitions in `src-tauri/src/commands.rs`, `src-tauri/src/events.rs`, or `src-tauri/src/types/`, register them in `src-tauri/src/lib.rs`, then regenerate.

### Bypassing the Command Boundary

**What happens:** UI code performs remote HTTP or filesystem behavior directly.
**Why it's wrong:** It bypasses Tauri capabilities, proxy/retry configuration, structured errors, and native logging.
**Do this instead:** Add a focused service operation under `src-tauri/src/`, expose an adapter in `src-tauri/src/commands.rs`, and consume the generated command from a component.

### Putting Durable State Only in Pinia

**What happens:** A feature stores important user/library state only in `src/store.ts`.
**Why it's wrong:** Pinia state disappears when the application closes.
**Do this instead:** Persist user settings through `src-tauri/src/config.rs` or library-specific data alongside `metadata.json`, then hydrate Pinia through a command.

## Error Handling

**Strategy:** Native layers add context with `anyhow`, command adapters convert errors into `CommandError`, and the frontend handles generated discriminated `Result` values.

**Patterns:**
- Add operation-specific context close to failing I/O in `src-tauri/src/hitomi_client.rs`, `src-tauri/src/download_manager.rs`, and `src-tauri/src/export.rs`.
- Convert command failures with `CommandError::from` in `src-tauri/src/commands.rs`.
- Log background-task error chains with `AnyhowErrorToStringChain` from `src-tauri/src/extensions.rs`.
- Check `result.status === 'error'` in Vue callers such as `src/AppContent.vue` and `src/store.ts`.
- Use typed lifecycle events for async background failures in `src-tauri/src/events.rs`.

## Cross-Cutting Concerns

**Logging:** Use `tracing` in native code; `src-tauri/src/logger.rs` fans output to console, optional rolling files, and the live frontend log viewer.
**Validation:** Validate external URL prefixes and command parameters at the IPC boundary in `src-tauri/src/commands.rs`; TypeScript strictness and generated command signatures guard frontend calls.
**Authentication:** There is no app identity. E-Hentai cookie fields are application configuration assembled in `src/utils.ts` and sent only to backend E-Hentai requests.
**Internationalization:** Put UI messages in `src/locales/en-US.json` and `src/locales/zh-CN.json`; initialize locale state in `src/main.ts`.
**Styling:** Use UnoCSS utilities and Naive UI components under `src/**/*.vue`; global theme overrides live in `src/App.vue`.

---

*Architecture analysis: 2026-07-29*
