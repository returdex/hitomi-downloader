# Phase 2: Release boundary and validation closure - Pattern Map

**Mapped:** 2026-07-29
**Files analyzed:** 8 个新建/修改文件（另含 Git tag 与里程碑审计操作）
**Analogs found:** 7 / 8

## File Classification

| 新建/修改文件 | 角色 | 数据流 | 最接近类比 | 匹配质量 |
|---|---|---|---|---|
| `src/panes/SearchPane.vue` | component | request-response / async state publication | `src/panes/DownloadingPane.vue` | exact |
| `src/panes/SearchPane.spec.ts` | test | request-response | `src/panes/SearchPane.vue` 的公开组件边界；仓库无既有测试 | no repository test analog |
| `package.json` | config | batch | `package.json` 现有 `build` 脚本 | role-match |
| `vite.config.ts` | config | transform | `vite.config.ts` 现有单一 Vite 配置 | exact |
| `pnpm-lock.yaml` | config | dependency resolution | 当前 `pnpm-lock.yaml` | exact / generated |
| `.planning/phases/02-release-boundary-validation/02-VALIDATION.md` | validation document | batch / evidence | `01-VERIFICATION.md` 的证据组织；字段以 `02-RESEARCH.md` 的 Nyquist 合约为准 | partial |
| `.planning/phases/02-release-boundary-validation/02-VERIFICATION.md` | verification document | batch / evidence | `.planning/phases/01-stabilization-cleanup/01-VERIFICATION.md` | exact |
| `.planning/phases/02-release-boundary-validation/02-01-SUMMARY.md` | summary document | event-driven / audit trail | `.planning/phases/01-stabilization-cleanup/01-01-SUMMARY.md` | exact |

> 规划文件自身应命名为 `02-01-PLAN.md`，直接复制
> `.planning/phases/01-stabilization-cleanup/01-01-PLAN.md` 的结构。Git 的
> annotated tag `v1.0` 是 release ref，不是仓库文件；重跑审计会更新
> `.planning/v1.0-MILESTONE-AUDIT.md`。

## Pattern Assignments

### `src/panes/SearchPane.vue`（component, request-response）

**主要类比：** `src/panes/DownloadingPane.vue`

**导入与本地状态模式**（`src/panes/SearchPane.vue:1-22`）：

```typescript
<script setup lang="tsx">
import { computed, nextTick, ref, watch } from 'vue'
import { commands, Suggestion } from '../bindings.ts'
import { SelectOption, useMessage, useNotification } from 'naive-ui'
import { useStore } from '../store.ts'

const { t } = useI18n()
const store = useStore()
const notification = useNotification()
const currentPage = ref<number>(1)
```

继续使用相对路径、显式 `.ts` 后缀、Composition API `ref`、真实 Pinia
store 和 Naive UI notification。不要为这个修复抽取新 service 或 store。

**成功后才提交状态的最近类比**（`src/panes/DownloadingPane.vue:82-92`）：

```typescript
async function syncPickedComic() {
  if (store.pickedComic === undefined) {
    return
  }
  const result = await commands.getSyncedComic(store.pickedComic)
  if (result.status === 'error') {
    console.error(result.error)
    return
  }
  store.pickedComic = result.data
}
```

同文件的对象内更新版本位于 `src/panes/DownloadingPane.vue:94-108`：

```typescript
const result = await commands.getSyncedComic(comic)
if (result.status === 'error') {
  console.error(result.error)
  return
}
Object.assign(comic, { ...result.data })
```

共同规则是：先 await 命令，错误分支立即返回，只有 success 分支才发布
响应关联的 UI/store 状态。

**当前缺陷与应复制的修正形状**（当前代码 `src/panes/SearchPane.vue:74-92`）：

```typescript
async function handlePageChange(pageNum: number) {
  if (store.searchResult === undefined) {
    return
  }

  currentPage.value = pageNum
  const result = await commands.getPage(store.searchResult.ids, pageNum)
  if (result.status === 'error') {
    console.error(result.error)
    notification.error({
      title: () => t('search_pane.search_failed'),
      description: () => result.error.err_message,
    })
    return
  }

  store.searchResult = result.data
}
```

规划动作应把 `currentPage.value = pageNum` 移到 error guard 之后，并与
`store.searchResult = result.data` 相邻。请求前捕获现有 `ids`（或至少不在
成功前改动任何可见状态）。失败分支完整保留当前 console + localized
notification；不要清空旧结果，也不要回滚式二次赋值。

**成功分支的同组件类比**（`src/panes/SearchPane.vue:120-131`）：

```typescript
const result = await commands.getComic(comicId)
if (result.status === 'error') {
  console.error(result.error)
  notification.error({
    title: () => t('search_pane.comic_load_failed'),
    description: () => result.error.err_message,
  })
  return
}

store.pickedComic = result.data
store.currentTabName = 'comic'
```

这说明同一组件已经采用“错误不提交、成功时连续提交相关状态”的约定。

**模板契约**（`src/panes/SearchPane.vue:412-417`）：

```vue
<n-pagination
  v-if="store.searchResult !== undefined"
  :page-count="store.searchResult.totalPage"
  :page="currentPage"
  @update:page="handlePageChange" />
```

测试必须经 `update:page` 事件覆盖这个公开契约，而不是只测复制出来的 helper。

---

### `src/panes/SearchPane.spec.ts`（test, request-response）

**类比：** 仓库没有 `*.test.*`、`*.spec.*`、测试依赖或测试配置；最接近的
真实契约是 `SearchPane.vue:74-92` 的 handler 与 `:page` /
`@update:page` 模板边界。文件应与被测组件同目录，符合研究给出的最小落点。

**导入模式（按现有源码相对路径约定）：**

```typescript
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import SearchPane from './SearchPane.vue'
import { commands } from '../bindings.ts'
import { useStore } from '../store.ts'
```

**状态初始化类比**（`src/store.ts:6-12`）：

```typescript
export const useStore = defineStore('store', () => {
  const currentTabName = ref<CurrentTabName>('search')
  const searchResult = ref<SearchResult>()
```

每个测试创建并激活新的 Pinia，使用真实 `useStore()` 写入 synthetic
`searchResult`；只 mock 外部 Tauri `commands.getPage` 和 Naive UI
notification，不 mock Pinia 的响应语义。

**核心测试形状：**

```typescript
const wrapper = mount(SearchPane, {
  global: {
    plugins: [pinia],
    stubs: {
      NPagination: {
        props: ['page', 'pageCount'],
        emits: ['update:page'],
        template:
          '<button data-test="pagination" @click="$emit(`update:page`, 2)">{{ page }}</button>',
      },
    },
  },
})

await wrapper.get('[data-test="pagination"]').trigger('click')
await flushPromises()
```

失败用例必须同时断言：

```typescript
expect(commands.getPage).toHaveBeenCalledWith(previousResult.ids, 2)
expect(wrapper.get('[data-test="pagination"]').text()).toBe('1')
expect(store.searchResult).toEqual(previousResult)
expect(notificationError).toHaveBeenCalledOnce()
```

成功用例必须断言分页显示为 `2` 且 `store.searchResult` 为 page-two result。
两条分支都要有，避免“永不翻页”也通过失败测试。fixture 不得包含真实 cookie、
配置或路径。

---

### `package.json`（config, batch）

**类比：** `package.json:6-10`

```json
"scripts": {
  "dev": "vite",
  "build": "vue-tsc --noEmit && vite build",
  "preview": "vite preview",
  "tauri": "tauri"
}
```

在相同 `scripts` 对象追加一条非 watch gate：

```json
"test": "vitest run"
```

保持现有两空格 JSON 格式与 pnpm 包管理器。新增 devDependencies 使用
`pnpm add -D vitest@4.1.10 @vue/test-utils@2.4.11 happy-dom@20.11.1`，
让 pnpm 同步修改 `package.json` 和 `pnpm-lock.yaml`，不要手改 lockfile。

最小命令模式：

- 聚焦：`pnpm test -- src/panes/SearchPane.spec.ts`
- 前端全量：`pnpm test && pnpm build`
- phase gate：再加 `cargo check --manifest-path src-tauri/Cargo.toml`

Phase 1 的现有构建证据来自
`.planning/phases/01-stabilization-cleanup/01-VERIFICATION.md:29-35`。

---

### `vite.config.ts`（config, transform）

**类比：** 当前文件自身，`vite.config.ts:1-31`

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
// ...
export default defineConfig(async () => ({
  plugins: [
    vue(),
    UnoCSS(),
    vueJsx({}),
    vueDevTools(),
    AutoImport({ /* ... */ }),
    Components({ resolvers: [NaiveUiResolver()] }),
  ],
```

保留这一个共享配置和全部插件，使测试与生产 `.vue`/TSX/自动组件解析一致。
按研究建议使用兼容 Vitest `test` 字段的 config typing，并在返回对象顶层加：

```typescript
test: {
  environment: 'happy-dom',
  restoreMocks: true,
},
```

不要建立第二套 Vue 编译配置。

---

### `pnpm-lock.yaml`（config, dependency resolution）

**类比：** 当前 lockfile。

只允许由上述 `pnpm add -D ...` 生成。验证 `package.json` 的三个 pinned
devDependencies 与 lockfile importer 一致；不要复制或手工拼接 lockfile 节点。

---

### `.planning/phases/02-release-boundary-validation/02-VALIDATION.md`

（validation document, batch/evidence）

**类比：** 仓库没有既有 `VALIDATION.md`。证据组织借用
`01-VERIFICATION.md:1-9,19-35`，Nyquist 专有字段严格采用
`02-RESEARCH.md` 的 “What `02-VALIDATION.md` Must Contain” 与
“Validation Architecture”。

**frontmatter 形状：**

```yaml
---
phase: 02
slug: release-boundary-validation
status: complete
wave_0_complete: true
nyquist_compliant: true
created: 2026-07-29
updated: 2026-07-29
---
```

规划时可以是 draft/false，但执行结束不得保留 draft、pending 或 false。

正文至少包含：

1. Test Infrastructure：框架版本、`vite.config.ts`、聚焦/全量命令、实测耗时。
2. Requirements → Test Map：REL-01、UI-01、VAL-01 每项有自动命令、
   测试/证据文件和最终 green 状态。
3. Wave 0 checklist：package script、config、spec、依赖全部完成。
4. Evidence：测试数、build/cargo 结果、tag name/object/target commit、tag 时
   clean-tree 观察、tag 中 debounce symbols 缺失证明。
5. Sign-off：无 watch flag、sampling gap 满足、full suite green、
   `nyquist_compliant: true` 与批准日期。

证据表沿用 `01-VERIFICATION.md:21-27` 的明确四列：

```markdown
| ID | Truth | Status | Evidence |
|----|-------|--------|----------|
| UI-01 | Failed pagination retains page and results; success updates both. | VERIFIED | `pnpm test -- src/panes/SearchPane.spec.ts` ... |
```

远程 tag 发布若未经授权可列 manual-only；本地 tag 内容检查必须自动化，不能
伪装为手工项。

---

### `.planning/phases/02-release-boundary-validation/02-VERIFICATION.md`

（verification document, batch/evidence）

**类比：** `.planning/phases/01-stabilization-cleanup/01-VERIFICATION.md`

**frontmatter**（类比行 1-9）：

```yaml
---
status: passed
phase: 02-release-boundary-validation
verified: 2026-07-29
plans: [02-01]
automated_checks:
  - pnpm test
  - pnpm build
  - cargo check --manifest-path src-tauri/Cargo.toml
---
```

复制正文栏目 `Result`、`Must-Haves Checked`、`Automated Checks`、
`Human Verification`、`Gaps`。Must-Haves 表必须按 REL-01/UI-01/VAL-01
逐行引用具体 commit/tag/test/document，而不是只写“reviewed”。

---

### `.planning/phases/02-release-boundary-validation/02-01-SUMMARY.md`

（summary document, event-driven/audit trail）

**类比：** `.planning/phases/01-stabilization-cleanup/01-01-SUMMARY.md`

复制其完整 frontmatter 分组：

```yaml
---
phase: 02-release-boundary-validation
plan: 01
subsystem: ui
tags: [vue, vitest, vue-test-utils, git, validation]

requires: []
provides:
  - Transactional pagination state publication
  - Focused automated UI regression coverage
  - Auditable clean v1.0 release boundary
affects: [search, tests, release, validation]

tech-stack:
  added: [vitest, vue-test-utils, happy-dom]
  patterns:
    - Publish page and result only after command success

key-files:
  created:
    - src/panes/SearchPane.spec.ts
    - .planning/phases/02-release-boundary-validation/02-VALIDATION.md
  modified:
    - src/panes/SearchPane.vue
    - package.json
    - pnpm-lock.yaml
    - vite.config.ts

requirements-completed: [REL-01, UI-01, VAL-01]
---
```

正文沿用 Phase 1 summary 的 Performance、Accomplishments、Task Commits、
Files Created/Modified、Decisions、Deviations、Issues、Verification、
Next Phase Readiness、Self-Check。REL-01 必须记录 stash 标识、release
commit、annotated tag object/target，以及 debounce 恢复后的检查结果。

## Planning Document Pattern

`02-01-PLAN.md` 应复制
`.planning/phases/01-stabilization-cleanup/01-01-PLAN.md:1-37` 的 frontmatter：

```yaml
---
phase: 02-release-boundary-validation
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - package.json
  - pnpm-lock.yaml
  - vite.config.ts
  - src/panes/SearchPane.vue
  - src/panes/SearchPane.spec.ts
  - .planning/phases/02-release-boundary-validation/02-VALIDATION.md
autonomous: false
requirements:
  - REL-01
  - UI-01
  - VAL-01
user_setup: []
must_haves:
  truths: []
  artifacts: []
  key_links: []
---
```

保留 Phase 1 的 XML section 结构：`objective`、`execution_context`、
`context`、`threat_model`、`tasks`、`verification`、`success_criteria`、
`output`。每个 task 保留 `files`、`read_first`、`action`、`verify`、
`acceptance_criteria`、`done`，并把 requirement ID 写进 truth/criteria。

## Shared Patterns

### 异步命令结果与错误处理

**来源：** `src/panes/SearchPane.vue:120-131`、
`src/panes/DownloadingPane.vue:82-108`

**适用：** `SearchPane.vue` 修复和对应测试

```typescript
const result = await commands.someCommand(input)
if (result.status === 'error') {
  console.error(result.error)
  // direct user action: existing localized notification/message
  return
}
// success-only state publication
store.someState = result.data
```

保持绑定生成的 discriminated result union，不增加 try/catch 或新错误类型。

### 真实 Pinia + 仅 mock 外部边界

**来源：** `src/store.ts:6-26` 与研究的组件测试架构

**适用：** `SearchPane.spec.ts`

用 fresh `createPinia`/`setActivePinia` 验证真实响应性；mock
`commands.getPage`、notification 与需要隔离的重型子组件。显式
`NPagination` stub 必须声明 `page`、`pageCount` 与 `update:page`。

### 验证证据

**来源：** `01-VERIFICATION.md:21-35`、
`01-01-SUMMARY.md:66-72,100-106`

**适用：** VALIDATION、VERIFICATION、SUMMARY

- 命令必须原样记录并标明 passed/failed。
- 已知 warning 与新失败分开记录。
- 每项 requirement 给出文件、命令、commit/tag 等可复查证据。
- summary 记录每个原子 task commit；verification 独立做 goal-backward 判断。

### 用户未提交 `src/AppContent.vue` debounce 的保护协议

**来源：** 当前 `git diff -- src/AppContent.vue` 与 REL-01；该 diff 包含
`onBeforeUnmount`、`saveConfigDebounceDelay`、`saveConfigTimer`、
`scheduleSaveConfig` 等延期实现。

**适用：** 所有 Phase 2 执行任务和 release boundary 操作

执行计划必须把以下保护步骤写成显式 checkpoint，不能让一般性的“清理工作树”
隐含处理用户补丁：

1. **前置指纹。** 运行 `git status --short` 与
   `git diff -- src/AppContent.vue`；保存 diff hash/文本证据，并确认该路径没有
   staged 变化。不要执行 `git reset --hard`、`git checkout --` 或
   `git restore src/AppContent.vue`。
2. **仅隔离该路径。** 用唯一消息执行 path-scoped named stash：
   `git stash push -m "phase-02-preserve-appcontent-debounce" -- src/AppContent.vue`。
   随即用 `git stash list` 和 `git stash show -p <captured-ref>` 核对补丁。
   不要依赖后续可能漂移的 `stash@{0}`；记录 stash object/ref。
3. **Phase 2 禁止触碰 AppContent。** `files_modified` 不列
   `src/AppContent.vue`；每个实现 commit 只 add 明确 Phase 2 路径，绝不使用
   `git add -A`、`git add .` 或包含该文件的批量提交。
4. **tag 前 clean boundary。** 测试/build/cargo 通过后，确认
   `git status --short` 对 release commit 无应用源码脏改动；从目标 commit
   建 annotated `v1.0` tag，并记录 tag object 与 peeled commit。
5. **tag 内容负向证明。** 对 `git show v1.0:src/AppContent.vue` 做
   symbol-negative 检查，至少确认不存在
   `saveConfigDebounceDelay|saveConfigTimer|scheduleSaveConfig|onBeforeUnmount`。
6. **先 apply 后核验，最后才 drop。** 用捕获的 stash ref 执行 apply；
   再比较恢复 diff 与前置指纹。只有补丁已完整恢复并准备在下一 milestone
   持久化时才 drop；若有冲突或差异，保留 stash 并停止，不自动解决或覆盖。
7. **恢复后不纳入 v1.0。** 恢复的 `AppContent.vue` 仍是用户/下一 milestone
   工作，不能进入 Phase 2 commit；打包/审计 v1.0 必须针对 tag/clean checkout，
   不能针对恢复 debounce 后的工作树。

本地 annotated tag 是研究假设下的最小验收。推送 tag 或发布远程 release
属于外部状态变更，只有用户明确授权时执行。

## No Analog Found

| 文件 | 角色 | 数据流 | 原因 |
|---|---|---|---|
| `src/panes/SearchPane.spec.ts` | test | request-response | 仓库扫描未发现任何 `*.test.*` / `*.spec.*` 文件；需采用研究指定的 Vitest + VTU 模式 |
| `02-VALIDATION.md` 的 Nyquist schema | validation document | batch/evidence | 仓库没有既有 VALIDATION；以 `02-RESEARCH.md` 列出的必填字段为权威，证据表样式复制 `01-VERIFICATION.md` |

## Metadata

**Analog search scope:** `src/**/*.vue`, `src/**/*.ts`, `package.json`,
`vite.config.ts`, `.planning/**/*.md`, 当前 Git status/diff

**Files scanned:** 代码与规划文件索引共 90+；深入读取 11 个强相关文件

**Strong analogs retained:** 5（SearchPane、DownloadingPane、store、
Phase 1 plan/summary/verification）

**Pattern extraction date:** 2026-07-29
