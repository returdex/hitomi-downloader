# Phase 2: Release boundary and validation closure - Research

**Researched:** 2026-07-29  
**Domain:** Vue UI state consistency, frontend component testing, Git release boundaries, Nyquist validation evidence  
**Confidence:** HIGH

## Summary

Phase 2 should remain a narrow closure phase. The pagination defect is caused by `handlePageChange` assigning `currentPage` before `commands.getPage` succeeds; on error, the function returns without changing `store.searchResult`, so the new page number is displayed with the previous page's comics. `[VERIFIED: src/panes/SearchPane.vue]` The minimal correction is transactional state publication: request the candidate page first, then assign both `currentPage` and `store.searchResult` only on success. `[VERIFIED: codebase analysis]`

The repository has no test script, test dependencies, test configuration, or `*.test.*`/`*.spec.*` files. `[VERIFIED: package.json, pnpm-lock.yaml, repository file scan]` Use Vitest with Vue Test Utils and happy-dom as a small Vite-native Wave 0. Vitest reads `vite.config.*` by default, and Vue Test Utils v2 targets Vue 3. `[CITED: https://vitest.dev/guide/]` `[CITED: https://test-utils.vuejs.org/guide/]`

The uncommitted `src/AppContent.vue` debounce is the only current working-tree modification and is explicitly excluded from v1.0. `[VERIFIED: git status --short, git diff -- src/AppContent.vue, .planning/REQUIREMENTS.md]` Preserve it with a path-scoped named stash while Phase 2 is implemented and validated, tag the resulting clean committed boundary, then reapply the stash for the next milestone. A stash is temporary transport, not the final preservation mechanism; once reapplied, the debounce should become a next-milestone commit. `[CITED: https://git-scm.com/docs/git-stash]` `[VERIFIED: .planning/ROADMAP.md]`

**Primary recommendation:** Implement success-only pagination state publication, add one focused `SearchPane` component test with Vitest/VTU, create complete `02-VALIDATION.md` evidence, record a clean tested candidate commit SHA while the debounce patch is isolated, and leave formal annotated `v1.0` creation to `$gsd-complete-milestone`.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REL-01 | Establish a clean v1.0 release boundary that excludes the uncommitted settings-save debounce while preserving that work for the next milestone. | Path-scoped stash/tag/reapply sequence, release-boundary checks, and runtime-state inventory. `[VERIFIED: .planning/REQUIREMENTS.md]` |
| UI-01 | Keep pagination state and displayed results consistent when loading a requested page fails. | Success-only state publication pattern and component-level failure/success tests. `[VERIFIED: .planning/REQUIREMENTS.md, src/panes/SearchPane.vue]` |
| VAL-01 | Add automated validation for the affected UI behavior and produce Nyquist validation evidence for the milestone. | Vitest/VTU Wave 0, requirement-to-test map, and required `02-VALIDATION.md` fields. `[VERIFIED: .planning/REQUIREMENTS.md, .planning/config.json, GSD VALIDATION.md template]` |
</phase_requirements>

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Pagination state consistency | Browser / Client | API / Backend | `SearchPane.vue` owns displayed page and result replacement; the Tauri command only supplies the requested page result. `[VERIFIED: src/panes/SearchPane.vue, src/bindings.ts]` |
| Automated UI validation | Browser / Client test harness | — | The behavior is observable through the Vue component's pagination event, store state, rendered page prop, and mocked command result. `[VERIFIED: src/panes/SearchPane.vue]` |
| Release boundary | Git/release metadata | — | A Git tag resolves to a committed object; uncommitted working-tree content is not part of the tagged commit. `[CITED: https://git-scm.com/docs/git-tag]` |
| Nyquist evidence | Planning artifacts | Test harness | `02-VALIDATION.md` records commands, sampling, requirement mapping, Wave 0 status, and sign-off backed by actual test output. `[VERIFIED: C:/Users/retur/.codex/get-shit-done/templates/VALIDATION.md]` |

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Vitest | 4.1.10, published/modified 2026-07-24 | Test runner, mocks, assertions | Current registry version; Vite-native and reads the existing Vite config. `[VERIFIED: npm registry]` `[CITED: https://vitest.dev/guide/]` |
| @vue/test-utils | 2.4.11, published/modified 2026-06-04 | Mount and interact with `SearchPane.vue` | Official Vue testing utility; v2 targets Vue 3. `[VERIFIED: npm registry]` `[CITED: https://test-utils.vuejs.org/guide/]` |
| happy-dom | 20.11.1, published/modified 2026-07-22 | Browser-like DOM for Node tests | Vitest supports it as a browser API environment; it is sufficient for this shallow component interaction. `[VERIFIED: npm registry]` `[CITED: https://vitest.dev/guide/environment.html]` |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Pinia | existing `^3.0.2` | Real `useStore()` state in the component test | Install a fresh active Pinia per test rather than mocking application state semantics. `[VERIFIED: package.json, src/store.ts]` |
| Vite Vue plugin | existing `^5.2.1` | Compile `.vue` files in tests | Reuse the current `vite.config.ts`; Vitest reads it by default. `[VERIFIED: package.json, vite.config.ts]` `[CITED: https://vitest.dev/guide/]` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| happy-dom | jsdom | jsdom provides broader browser API coverage; happy-dom is smaller/faster but officially documented as lacking some APIs. The tested pagination path does not require those missing APIs. `[CITED: https://vitest.dev/guide/environment.html]` |
| Component test | Pure helper-only unit test | A helper test would be easier but would not prove the `n-pagination` event, command failure, displayed `page`, and stored results stay aligned. `[VERIFIED: src/panes/SearchPane.vue]` |
| Vitest/VTU | Playwright/Cypress E2E | Full browser/Tauri E2E adds disproportionate setup for one client-state regression and is not present in this repository. `[VERIFIED: package.json, repository file scan]` |

**Installation:**

```bash
pnpm add -D vitest@4.1.10 @vue/test-utils@2.4.11 happy-dom@20.11.1
```

`vitest@4.1.10` requires Vite 6+ and Node 20+; the project uses Vite `^6.0.3` and the audited machine has Node 24.13.1. `[CITED: https://vitest.dev/guide/]` `[VERIFIED: package.json, node --version]`

## Architecture Patterns

### System Architecture Diagram

```text
User selects candidate page N
          |
          v
SearchPane.handlePageChange(N)
          |
          v
commands.getPage(existing ids, N) -----> Tauri/backend boundary
          |
          +---- error ----> retain currentPage + retain searchResult
          |                  + show existing error notification
          |
          +---- success ---> publish currentPage=N + replace searchResult
                             |
                             v
                 pagination and comic list agree
```

`commands.getPage` already returns an explicit success/error result union used by the component. `[VERIFIED: src/panes/SearchPane.vue, src/bindings.ts]`

### Recommended Project Structure

```text
package.json                         # add non-watch test scripts
vite.config.ts                       # add test environment/config typing
src/
└── panes/
    ├── SearchPane.vue               # success-only pagination publication
    └── SearchPane.spec.ts           # focused success and failure cases
.planning/phases/02-release-boundary-validation/
├── 02-VALIDATION.md                 # executable Nyquist contract/evidence
├── 02-VERIFICATION.md               # phase goal verification
└── 02-RESEARCH.md
```

### Pattern 1: Publish Related UI State Atomically

**What:** Treat `pageNum` as a candidate value. Do not change visible state until its corresponding result arrives successfully. `[VERIFIED: codebase analysis]`  
**When to use:** Whenever one visible value identifies another asynchronously loaded value.

```typescript
// Source: repository-specific correction derived from src/panes/SearchPane.vue
async function handlePageChange(pageNum: number) {
  const existing = store.searchResult
  if (existing === undefined) return

  const result = await commands.getPage(existing.ids, pageNum)
  if (result.status === 'error') {
    console.error(result.error)
    notification.error({
      title: () => t('search_pane.search_failed'),
      description: () => result.error.err_message,
    })
    return
  }

  currentPage.value = pageNum
  store.searchResult = result.data
}
```

This preserves the current notification behavior and changes only the publication order. `[VERIFIED: src/panes/SearchPane.vue]`

### Pattern 2: Test Observable Component Behavior

**What:** Mount the component with a real Pinia, mock `commands.getPage`, emit `update:page` from a pagination stub, and assert the pagination `page` prop and store result after the promise settles. `[CITED: https://test-utils.vuejs.org/guide/essentials/easy-to-test.html]`  
**When to use:** For both the rejected request (state unchanged) and successful request (page and result change together).

```typescript
// Source: https://test-utils.vuejs.org/api/ and repository component contract
const wrapper = mount(SearchPane, {
  global: {
    plugins: [pinia],
    stubs: {
      NPagination: {
        props: ['page', 'pageCount'],
        emits: ['update:page'],
        template: '<button data-test="pagination" @click="$emit(`update:page`, 2)">{{ page }}</button>',
      },
    },
  },
})

await wrapper.get('[data-test="pagination"]').trigger('click')
await flushPromises()
expect(wrapper.get('[data-test="pagination"]').text()).toBe('1')
expect(store.searchResult).toEqual(previousResult)
```

Vue Test Utils supports `global.plugins`, component stubs, `mount`, and DOM-triggered interaction. `[CITED: https://test-utils.vuejs.org/api/]`

### Pattern 3: Reversible Release Isolation

**What:** Use a named, path-scoped stash to remove only the deferred `AppContent.vue` patch, build/test/commit Phase 2, verify cleanliness, record the full candidate commit SHA, then reapply the saved patch. `$gsd-complete-milestone` creates the annotated tag after final closure. `[CITED: https://git-scm.com/docs/git-stash]`

```bash
git diff --check -- src/AppContent.vue
git stash push -m "deferred: settings-save debounce for post-v1.0" -- src/AppContent.vue
# implement, test, validate and commit Phase 2
git status --short
candidate_sha=$(git rev-parse HEAD)
git show "$candidate_sha:src/AppContent.vue"
git stash apply stash^{/deferred: settings-save debounce for post-v1.0}
```

Before tagging, `git status --short` must be empty; after applying, compare `git diff -- src/AppContent.vue` with the originally reviewed patch before deleting the stash. `[VERIFIED: release-boundary control design]`

### Anti-Patterns to Avoid

- **Optimistic page assignment without rollback:** It recreates the audited mismatch. Assign after success instead. `[VERIFIED: src/panes/SearchPane.vue, .planning/v1.0-MILESTONE-AUDIT.md]`
- **Clearing results while loading:** It avoids a mismatch but creates unnecessary UI flicker and changes behavior beyond UI-01. `[VERIFIED: phase scope analysis]`
- **Testing private refs/functions directly:** Test the pagination event and observable props/store output, not `<script setup>` internals. `[CITED: https://test-utils.vuejs.org/guide/essentials/easy-to-test.html]`
- **Using watch-mode at validation gates:** `vitest` watches by default in development; use `vitest run`/`pnpm test` configured as a one-shot command. `[CITED: https://vitest.dev/guide/]` `[VERIFIED: GSD VALIDATION.md template]`
- **Tagging with a dirty tree and treating that as evidence:** A tag identifies a commit, but a dirty tree can still be accidentally packaged outside the tagged checkout. Build and audit from the clean boundary. `[CITED: https://git-scm.com/docs/git-tag]` `[VERIFIED: .planning/v1.0-MILESTONE-AUDIT.md]`
- **Dropping the stash immediately after apply:** Keep it until the reapplied diff is checked and durably committed to the next milestone. `[CITED: https://git-scm.com/docs/git-stash]`

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Vue SFC compilation/test runner | Custom Node loader | Vitest + existing Vite Vue plugins | Vitest consumes Vite config and recognizes `.test.`/`.spec.` files. `[CITED: https://vitest.dev/guide/]` |
| Component mounting and event simulation | Manual Vue app/DOM harness | Vue Test Utils | Official Vue 3 mounting, plugins, stubs, and wrapper APIs. `[CITED: https://test-utils.vuejs.org/guide/]` |
| Browser API emulation | Ad-hoc globals | happy-dom test environment | Vitest supplies supported DOM environments through configuration. `[CITED: https://vitest.dev/guide/environment.html]` |
| Deferred-patch preservation | Copy/paste or an untracked backup | Named Git stash followed by next-milestone commit | Git records the working/index state and supports path-scoped restoration. `[CITED: https://git-scm.com/docs/git-stash]` |
| Candidate identity | A mutable branch name | Full commit SHA | The SHA is immutable evidence for Phase 2; milestone completion later creates the release tag. |

**Key insight:** The phase needs proof of behavior and provenance, not new application architecture.

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — the debounce changes save timing only; no database key/schema rename or migration is involved. `[VERIFIED: git diff -- src/AppContent.vue]` | None. |
| Live service config | None found — no CI workflow or external release configuration exists in the repository scan. `[VERIFIED: repository file scan]` | Tag/push/release publication outside the local repository remains an execution-time user/release-owner action. |
| OS-registered state | None — no task/service registration is touched by the scoped Vue/Git changes. `[VERIFIED: phase scope and repository scan]` | None. |
| Secrets/env vars | None — the patch and pagination change introduce no secret/env names. `[VERIFIED: git diff -- src/AppContent.vue, src/panes/SearchPane.vue]` | Do not print existing config/cookies in tests or validation logs. |
| Build artifacts / installed packages | `node_modules` must gain three dev dependencies; the v1.0 artifact must be built from the tagged clean tree, not the post-tag tree with reapplied debounce. `[VERIFIED: package.json, release-boundary analysis]` | Run `pnpm install`, `pnpm test`, and `pnpm build`; if packaging occurs, checkout/build the tag explicitly. |

The canonical post-file-edit check is: after the tag is created, `git show v1.0:src/AppContent.vue` must lack `saveConfigDebounceDelay`, `saveConfigTimer`, `scheduleSaveConfig`, and the debounce `onBeforeUnmount` path, while the preserved next-milestone patch contains them. `[VERIFIED: current AppContent.vue diff]`

## Common Pitfalls

### Pitfall 1: Failure Test Does Not Await the Async Handler

**What goes wrong:** Assertions run before `commands.getPage` resolves.  
**Why it happens:** Vue event triggers and mocked promises schedule asynchronous updates.  
**How to avoid:** Await the trigger and flush pending promises before assertions. `[CITED: https://test-utils.vuejs.org/guide/essentials/a-crash-course.html]`  
**Warning signs:** The test passes even when the success and error mock results are swapped.

### Pitfall 2: Auto-Resolved Naive UI Components Obscure the Pagination Contract

**What goes wrong:** A shallow stub does not expose or emit `update:page`, so no application behavior is exercised.  
**Why it happens:** `n-pagination` is auto-resolved through `unplugin-vue-components`; generic stubs do not model its event. `[VERIFIED: vite.config.ts, src/panes/SearchPane.vue]`  
**How to avoid:** Supply an explicit `NPagination` stub with `page`, `pageCount`, and `update:page`.  
**Warning signs:** `commands.getPage` is never called.

### Pitfall 3: Mocking the Entire Store

**What goes wrong:** The test proves a mock object, not Pinia reactivity used by the component.  
**Why it happens:** Full store mocking initially appears simpler.  
**How to avoid:** Create/set an active Pinia for each test and seed `store.searchResult`; mock only the external Tauri command and notification hooks. `[VERIFIED: src/store.ts]`  
**Warning signs:** Pagination props do not update after store mutation.

### Pitfall 4: Stash Reference Drift

**What goes wrong:** A later stash changes `stash@{0}`, and the wrong patch is applied or dropped.  
**Why it happens:** Ordinal stash references are mutable.  
**How to avoid:** Use a unique message, capture the created stash object/reference, inspect `git stash list`/`git stash show -p`, use `apply` before `drop`, and verify the restored diff. `[CITED: https://git-scm.com/docs/git-stash]`  
**Warning signs:** The restored `AppContent.vue` diff differs from the pre-isolation diff.

### Pitfall 5: VALIDATION.md Is a Plan, Not Evidence

**What goes wrong:** The file exists but retains `status: draft`, `nyquist_compliant: false`, pending rows, or Wave 0 gaps after execution.  
**Why it happens:** The planning template is never updated with real results.  
**How to avoid:** During execution, record exact passing commands, mark per-task rows green, set `wave_0_complete: true`, set `nyquist_compliant: true`, and add approval date only after the commands pass. `[VERIFIED: GSD VALIDATION.md template, audit-milestone workflow]`  
**Warning signs:** The milestone audit classifies the phase partial rather than compliant.

## Code Examples

### Minimal Vitest Configuration

```typescript
// Source: https://vitest.dev/guide/ and https://vitest.dev/guide/environment.html
import { defineConfig } from 'vitest/config'
// Keep the existing Vue/Vite plugins in the exported configuration.
export default defineConfig({
  test: {
    environment: 'happy-dom',
    restoreMocks: true,
  },
})
```

Prefer adding the `test` block to the existing configuration (with compatible config typing) so Vue, JSX, auto-import, component resolver, and UnoCSS behavior stays aligned. Vitest reads `vite.config.*` by default. `[CITED: https://vitest.dev/guide/]` `[VERIFIED: vite.config.ts]`

### Required Failure Assertion

```typescript
// Source: repository requirement UI-01
expect(commands.getPage).toHaveBeenCalledWith(previousResult.ids, 2)
expect(pagination.attributes('page')).toBe('1')
expect(store.searchResult).toEqual(previousResult)
expect(notificationError).toHaveBeenCalledOnce()
```

The exact wrapper accessor may differ depending on the explicit pagination stub, but the four assertions are the required behavioral evidence. `[VERIFIED: UI-01 and src/panes/SearchPane.vue]`

### Required Success Assertion

```typescript
// Source: repository requirement UI-01
expect(pagination.attributes('page')).toBe('2')
expect(store.searchResult).toEqual(pageTwoResult)
```

Testing both branches prevents a “fix” that simply blocks all pagination changes. `[VERIFIED: phase behavior analysis]`

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Separate Jest transforms for Vue SFCs | Vite-native Vitest using existing Vite plugins | Current Vitest/Vue Test Utils guidance | Minimal configuration for this existing Vite app. `[CITED: https://test-utils.vuejs.org/installation/]` |
| Vue Test Utils v1 / Vue 2 APIs | Vue Test Utils v2 for Vue 3 | VTU major-version boundary | Do not use `createLocalVue` or other v1 examples. `[CITED: https://test-utils.vuejs.org/guide/]` |
| Vitest 3-era assumptions | Vitest 4.1.10 with Node 20+/Vite 6+ requirements | Vitest 4/current release | The repository's Vite and machine Node satisfy the documented floor. `[VERIFIED: npm registry, package.json, node --version]` `[CITED: https://vitest.dev/guide/]` |

**Deprecated/outdated:**

- Vue Test Utils v1 documentation applies to Vue 2 and must not guide this Vue 3 test. `[CITED: https://test-utils.vuejs.org/guide/]`
- Watch-mode `vitest` is unsuitable as a phase gate; use `vitest run`. `[CITED: https://vitest.dev/guide/]`

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Phase 2 records a verified clean candidate SHA; milestone completion owns the formal tag. | Open Questions (resolved) | No tag is created or asserted during Phase 2. |
| A2 | Phase 2 validation may satisfy milestone closure without a retroactive Phase 1 validation file. | Open Questions | The rerun audit may continue to classify Phase 1 as Nyquist-missing. |
| A3 | The focused component test should complete in under 30 seconds. | Validation Architecture | Sampling cadence may need a higher measured latency after dependencies are installed. |

## Open Questions (RESOLVED)

1. **Who creates the formal v1.0 tag? — RESOLVED**
   - Decision: Phase 2 creates no tag. It records a clean, tested, auditable candidate commit SHA; `$gsd-complete-milestone` creates the formal annotated `v1.0` tag only after final verification, audit closure, and archival. Remote publication remains outside Phase 2 authorization.

2. **Should Phase 1 receive a retroactive `01-VALIDATION.md`? — RESOLVED**
   - Decision: Yes. The milestone audit explicitly marks Phase 1 Nyquist evidence missing, so Phase 2 backfills `01-VALIDATION.md` from existing Phase 1 summary/verification build and check evidence. It must be labeled retrospective, distinguish recorded historical evidence from any rerun, and must not invent task-time measurements or tests.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Vitest/Vite | ✓ | 24.13.1 | — `[VERIFIED: node --version]` |
| pnpm | dependency install/test scripts | ✓ | 9.5.0 | `pnpm.cmd` under PowerShell execution-policy restrictions. `[VERIFIED: pnpm.cmd --version]` |
| Git | stash/tag boundary | ✓ | 2.53.0.windows.1 | — `[VERIFIED: git --version]` |
| Cargo/Rust | existing full phase regression check | ✓ | cargo 1.95.0 / rustc 1.95.0 | — `[VERIFIED: cargo --version, rustc --version]` |
| Vitest | UI automated validation | ✗ (not installed in project) | registry 4.1.10 | Install in Wave 0. `[VERIFIED: package.json, pnpm-lock.yaml, npm registry]` |
| Vue Test Utils | component mounting | ✗ (not installed in project) | registry 2.4.11 | Install in Wave 0. `[VERIFIED: package.json, pnpm-lock.yaml, npm registry]` |
| happy-dom | DOM environment | ✗ (not installed in project) | registry 20.11.1 | jsdom is viable if a missing DOM API is encountered. `[VERIFIED: package.json, pnpm-lock.yaml, npm registry]` `[CITED: https://vitest.dev/guide/environment.html]` |

**Missing dependencies with no fallback:** None; Wave 0 installation is required before implementation tests.  
**Missing dependencies with fallback:** happy-dom may be replaced with jsdom only if the focused component test exposes an unsupported browser API. `[CITED: https://vitest.dev/guide/environment.html]`

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Vitest 4.1.10 + Vue Test Utils 2.4.11 + happy-dom 20.11.1 `[VERIFIED: npm registry]` |
| Config file | `vite.config.ts` (add `test.environment`) `[VERIFIED: existing Vite config]` |
| Quick run command | `pnpm test -- src/panes/SearchPane.spec.ts` |
| Full suite command | `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml` |

Set `"test": "vitest run"` in `package.json`; this guarantees a non-watch gate. `[CITED: https://vitest.dev/guide/]`

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| REL-01 | Tagged v1.0 `AppContent.vue` excludes debounce symbols and tag resolves to intended release commit | Git smoke/assertion | `git show v1.0:src/AppContent.vue` plus symbol-negative `rg`; `git status --short` captured before tagging | ❌ Wave 0/plan task |
| UI-01 | Failed page load retains both current displayed page and previous result; successful load advances both | Vue component test | `pnpm test -- src/panes/SearchPane.spec.ts` | ❌ Wave 0 |
| VAL-01 | Automated commands pass and Nyquist document records green mapping/sign-off | Artifact/schema review + full suite | `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml` | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `pnpm test -- src/panes/SearchPane.spec.ts`
- **Per wave merge:** `pnpm test && pnpm build`
- **Phase gate:** `pnpm test && pnpm build && cargo check --manifest-path src-tauri/Cargo.toml`; then verify the clean candidate SHA and rerun the milestone audit. Formal tagging follows in milestone completion. `[VERIFIED: Phase 1 verification precedent, Phase 2 roadmap]`
- **Max feedback latency:** Target under 30 seconds for the focused component test. `[ASSUMED]`

### Wave 0 Gaps

- [ ] `package.json` — add `"test": "vitest run"` and the three pinned dev dependencies.
- [ ] `vite.config.ts` — add Vitest test configuration using happy-dom.
- [ ] `src/panes/SearchPane.spec.ts` — mock Tauri bindings/notification hooks and cover failure plus success.
- [ ] `.planning/phases/02-release-boundary-validation/02-VALIDATION.md` — instantiate the GSD template with REL-01/UI-01/VAL-01 rows.
- [ ] Dependency install: `pnpm add -D vitest@4.1.10 @vue/test-utils@2.4.11 happy-dom@20.11.1`.

### What `02-VALIDATION.md` Must Contain

- Frontmatter with `phase: 02`, slug, execution status, `wave_0_complete`, `nyquist_compliant`, and dates. `[VERIFIED: GSD VALIDATION.md template]`
- Test infrastructure table with pinned framework versions, config path, exact one-shot quick/full commands, and measured runtime. `[VERIFIED: GSD VALIDATION.md template]`
- Per-task verification rows mapping every planned task to REL-01, UI-01, or VAL-01, including secure behavior, test type, command, file existence, and final green/red state. `[VERIFIED: GSD VALIDATION.md template]`
- Wave 0 checklist updated from missing to complete after files/dependencies exist. `[VERIFIED: GSD VALIDATION.md template]`
- Manual-only table only for actions that cannot be automated (for example, authorized remote tag publication); automated local tag-content checks must not be listed as manual-only. `[VERIFIED: GSD VALIDATION.md template, repository capability]`
- Sign-off showing no watch flags, no three-task sampling gap, full suite green, feedback latency, compliance flag, and approval date. `[VERIFIED: GSD VALIDATION.md template]`
- Concrete output/evidence: passing test count, build result, cargo-check result, release tag name/object/commit, clean-tree observation at tag time, and proof debounce symbols are absent from tagged `AppContent.vue`. `[VERIFIED: VAL-01 and release-boundary analysis]`

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No authentication behavior changes. `[VERIFIED: scoped diffs/requirements]` |
| V3 Session Management | no | No session behavior changes. `[VERIFIED: scoped diffs/requirements]` |
| V4 Access Control | no | No authorization boundary changes. `[VERIFIED: scoped diffs/requirements]` |
| V5 Input Validation | yes, minimally | Treat candidate page as untrusted UI input and publish only backend-successful results; existing backend command result union remains the authority. `[VERIFIED: src/panes/SearchPane.vue]` |
| V6 Cryptography | no | No cryptographic operation is introduced. `[VERIFIED: scoped diffs/requirements]` |

### Known Threat Patterns for Vue/Tauri Release Closure

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Release candidate derived from dirty tree | Tampering | Require clean status, test/build before recording the full candidate SHA, and inspect candidate contents. `[VERIFIED: release-boundary analysis]` |
| Test logs expose mocked or real E-Hentai config/cookies | Information Disclosure | Use synthetic fixtures; never print `store.config`, cookies, or raw secret-bearing command data. `[VERIFIED: Phase 1 threat model and src/utils.ts usage]` |
| Async stale/out-of-order state publication | Tampering (integrity) | For this scoped defect, publish page and result only after success; do not add concurrency behavior not required by UI-01. `[VERIFIED: src/panes/SearchPane.vue, scope analysis]` |

## Sources

### Primary (HIGH confidence)

- Repository: `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/v1.0-MILESTONE-AUDIT.md`, Phase 1 plan/summary/verification — scope, gaps, and prior evidence.
- Repository: `src/panes/SearchPane.vue`, `src/AppContent.vue`, `src/store.ts`, `vite.config.ts`, `package.json`, `pnpm-lock.yaml` — implementation and tooling evidence.
- npm registry — Vitest 4.1.10, Vue Test Utils 2.4.11, happy-dom 20.11.1 and modification timestamps.
- https://vitest.dev/guide/ — installation, compatibility, configuration, file naming, and one-shot execution.
- https://vitest.dev/guide/environment.html — happy-dom/jsdom environment behavior and caveats.
- https://test-utils.vuejs.org/guide/ — official Vue 3 component testing scope.
- https://test-utils.vuejs.org/api/ — mount, plugins, shallow/stub APIs.
- https://git-scm.com/docs/git-stash — path-scoped reversible working-tree preservation.
- https://git-scm.com/docs/git-tag — annotated release tags.
- Installed GSD `templates/VALIDATION.md` and `workflows/audit-milestone.md` — Nyquist artifact schema and audit classification.

### Secondary (MEDIUM confidence)

- None.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — current registry versions and official docs verified against the existing Vite/Vue project.
- Architecture: HIGH — defect and correction follow directly from the current component state transitions.
- Release boundary: HIGH — current diff is isolated to one tracked file and Git behavior is documented officially.
- Pitfalls: HIGH — derived from current code/config and official test/Git behavior.
- Validation evidence: HIGH — matched to the installed GSD template and milestone-audit parser expectations.

**Research date:** 2026-07-29  
**Valid until:** 2026-08-05 for fast-moving npm versions; repository-specific findings remain valid until relevant files change.
