<script setup lang="tsx">
import { computed, nextTick, ref, watch } from 'vue'
import { commands, Suggestion } from '../bindings.ts'
import { SelectOption, useMessage, useNotification } from 'naive-ui'
import { open } from '@tauri-apps/plugin-dialog'
import ComicCard from '../components/ComicCard.vue'
import { useStore } from '../store.ts'
import { buildEhentaiCookie, hasEhentaiCookie, useI18n } from '../utils.ts'
import { PhMagnifyingGlass, PhArrowRight, PhFolderOpen } from '@phosphor-icons/vue'
import FloatLabelInput from '../components/FloatLabelInput.vue'

const { t } = useI18n()

const store = useStore()

const message = useMessage()
const notification = useNotification()

const searchInput = ref<string>('')
const searchInputRef = ref<InstanceType<typeof FloatLabelInput>>()
const comicIdInput = ref<string>('')
const currentPage = ref<number>(1)
const comicCardContainerRef = ref<HTMLElement>()
const {
  onSuggestionSelect,
  getSearchSuggestions,
  suggestionOptions,
  suggestionsShowing,
  dropdownFocused,
  dropdownX,
  dropdownY,
} = useSuggestion()

const searching = ref<boolean>(false)
const importingEhentaiFavorites = ref<boolean>(false)

watch(
  () => store.searchResult,
  () => {
    if (comicCardContainerRef.value !== undefined) {
      comicCardContainerRef.value.scrollTo({ top: 0, behavior: 'instant' })
    }
  },
)

async function search(query: string, pageNum: number) {
  if (searching.value) {
    message.warning(() => t('search_pane.searching_warning'))
    return
  }

  searchInput.value = query
  currentPage.value = pageNum
  store.currentTabName = 'search'
  searching.value = true
  suggestionsShowing.value = false

  // TODO: support sort by popularity
  const result = await commands.search(query, pageNum, false)
  if (result.status === 'error') {
    console.error(result.error)
    notification.error({
      title: () => t('search_pane.search_failed'),
      description: () => result.error.err_message,
    })
    searching.value = false
    return
  }
  store.searchResult = result.data

  searching.value = false
}

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

function getComicIdFromComicIdInput(): number | undefined {
  const comicIdString = comicIdInput.value.trim()
  // if it is a number, return it directly
  const comicId = parseInt(comicIdString)
  if (!isNaN(comicId)) {
    return comicId
  }
  // otherwise, extract it from the url
  const regex = /-(\d+).html/
  const match = comicIdString.match(regex)
  if (match === null || match[1] === null) {
    return
  }
  return parseInt(match[1])
}

async function pickComic() {
  const comicId = getComicIdFromComicIdInput()
  if (comicId === undefined) {
    notification.error({
      title: () => t('search_pane.comic_id_invalid'),
      description: () => t('search_pane.enter_comic_id_or_url'),
    })
    return
  }

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
}

async function selectEhentaiDownloadDir() {
  const selectedDirPath = await open({ directory: true })
  if (selectedDirPath === null) {
    return
  }
  if (store.config !== undefined) {
    store.config.ehentaiFavoritesDownloadDir = selectedDirPath
  }
}

async function importEhentaiFavorites() {
  if (importingEhentaiFavorites.value) {
    return
  }
  if (
    store.config === undefined ||
    !hasEhentaiCookie(store.config) ||
    store.config.ehentaiFavoritesUrl.trim() === '' ||
    store.config.ehentaiFavoritesDownloadDir === ''
  ) {
    notification.error({
      title: () => t('search_pane.ehentai_import_invalid'),
    })
    return
  }

  importingEhentaiFavorites.value = true
  const result = await commands.importEhentaiFavorites(
    buildEhentaiCookie(store.config),
    store.config.ehentaiFavoritesUrl.trim(),
    store.config.ehentaiFavoritesDownloadDir,
    store.config.ehentaiFavoritesLimit,
    store.config.ehentaiFavoritesKnownIds,
    false,
  )
  importingEhentaiFavorites.value = false

  if (result.status === 'error') {
    console.error(result.error)
    notification.error({
      title: () => result.error.err_title,
      description: () => result.error.err_message,
    })
    return
  }

  store.config.ehentaiFavoritesKnownIds = Array.from(
    new Set([...store.config.ehentaiFavoritesKnownIds, ...result.data.foundIds]),
  )

  const { foundCount, queuedCount, failedCount, skippedKnownCount } = result.data
  notification.success({
    title: () => t('search_pane.ehentai_import_done'),
    description: () =>
      t('search_pane.ehentai_import_summary', {
        found: foundCount,
        queued: queuedCount,
        skipped: skippedKnownCount,
        failed: failedCount,
      }),
  })
}

function handleSearchInputKeydown(e: KeyboardEvent) {
  if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
    dropdownFocused.value = suggestionsShowing.value
  } else if (e.key === 'Enter') {
    if (!dropdownFocused.value) {
      search(searchInput.value.trim(), 1)
    }
  } else if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
    e.stopPropagation()
  }
}

function handleSearchInputKeyup(e: KeyboardEvent) {
  const { key } = e
  if (key !== 'Enter' && key !== 'ArrowUp' && key !== 'ArrowDown' && key !== 'ArrowLeft' && key !== 'ArrowRight') {
    getSearchSuggestions()
  }
}

function useSuggestion() {
  const suggestionsShowing = ref<boolean>(false)
  const suggestions = ref<Suggestion[]>([])
  const dropdownFocused = ref<boolean>(false)

  const dropdownX = computed<number>(() => {
    const x = searchInputRef.value?.NInputRef?.inputElRef?.getBoundingClientRect().left
    return x ? x - 2 : 0
  })

  const dropdownY = computed<number>(() => {
    const y = searchInputRef.value?.NInputRef?.inputElRef?.getBoundingClientRect().bottom
    return y ? y - 2 : 0
  })

  watch(suggestionsShowing, (showing) => {
    if (!showing) {
      dropdownFocused.value = false
    }
  })

  const suggestionOptions = computed<SelectOption[]>(() => {
    return suggestions.value.map((suggestion) => {
      const { s, n, t } = suggestion
      return {
        label: () => (
          <div class="flex">
            <span class="font-bold">{s}</span>
            <span class="text-gray-5 px-1">({n})</span>
            <span class="ml-auto text-gray-5">{t}</span>
          </div>
        ),
        key: `${n}:${s.replace(' ', '_')}`,
      }
    })
  })

  async function getSearchSuggestions() {
    suggestionsShowing.value = false
    let lastWord = searchInput.value.split(' ').pop()
    if (lastWord === undefined || lastWord === '') {
      suggestions.value = []
      return
    }

    if (lastWord.startsWith('-')) {
      lastWord = lastWord.substring(1)
    }

    const result = await commands.getSearchSuggestions(lastWord)
    if (result.status === 'error') {
      console.error(result.error)
      return
    }

    suggestions.value = result.data
    suggestionsShowing.value = true
  }

  async function onSuggestionSelect(key: string) {
    const words = searchInput.value.split(' ')
    const isNegative = words.pop()?.startsWith('-')
    if (isNegative === true) {
      words.push(`-${key}`)
    } else {
      words.push(key)
    }

    searchInput.value = words.join(' ') + ' '
    suggestionsShowing.value = false

    await nextTick()
    searchInputRef.value?.NInputRef?.scrollTo({
      behavior: 'smooth',
      left: 10000,
    })
  }

  return {
    suggestionsShowing,
    suggestionOptions,
    getSearchSuggestions,
    onSuggestionSelect,
    dropdownFocused,
    dropdownX,
    dropdownY,
  }
}

defineExpose({ search })
</script>

<template>
  <div class="h-full flex flex-col gap-2">
    <n-input-group class="box-border px-2 pt-2">
      <FloatLabelInput
        :label="t('search_pane.search_by_query')"
        ref="searchInputRef"
        size="small"
        v-model:value="searchInput"
        clearable
        @keyup="handleSearchInputKeyup"
        @keydown="handleSearchInputKeydown" />
      <n-dropdown
        v-if="suggestionsShowing"
        :x="dropdownX"
        :y="dropdownY"
        size="small"
        :show="suggestionsShowing"
        :options="suggestionOptions"
        @select="onSuggestionSelect"
        @clickoutside="() => (suggestionsShowing = false)" />
      <n-button :loading="searching" type="primary" class="w-15%" size="small" @click="search(searchInput.trim(), 1)">
        <template #icon>
          <n-icon size="22">
            <PhMagnifyingGlass />
          </n-icon>
        </template>
      </n-button>
    </n-input-group>

    <n-input-group class="box-border px-2">
      <FloatLabelInput
        :label="t('search_pane.search_by_id')"
        class="text-align-left"
        size="small"
        v-model:value="comicIdInput"
        clearable
        @keydown.enter="pickComic" />
      <n-button type="primary" class="w-15%" size="small" @click="pickComic">
        <template #icon>
          <n-icon size="22">
            <PhArrowRight />
          </n-icon>
        </template>
      </n-button>
    </n-input-group>

    <n-collapse class="box-border px-2">
      <n-collapse-item :title="t('search_pane.ehentai_favorites')" name="ehentai-favorites">
        <div v-if="store.config !== undefined" class="flex flex-col gap-2">
          <n-checkbox v-model:checked="store.config.ehentaiFavoritesAutoCheck">
            {{ t('search_pane.ehentai_auto_check') }}
          </n-checkbox>
          <span class="text-xs text-gray-500">{{ t('search_pane.ehentai_cookie_settings_hint') }}</span>
          <FloatLabelInput
            :label="t('search_pane.ehentai_favorites_url')"
            size="small"
            v-model:value="store.config.ehentaiFavoritesUrl"
            clearable />
          <n-input-group>
            <n-input-group-label size="small">{{ t('common.download_directory') }}</n-input-group-label>
            <n-input
              v-model:value="store.config.ehentaiFavoritesDownloadDir"
              size="small"
              readonly
              @click="selectEhentaiDownloadDir" />
            <n-button class="w-9" size="small" @click="selectEhentaiDownloadDir">
              <template #icon>
                <n-icon size="20">
                  <PhFolderOpen />
                </n-icon>
              </template>
            </n-button>
          </n-input-group>
          <div class="flex gap-2">
            <n-input-number
              class="w-32"
              v-model:value="store.config.ehentaiFavoritesLimit"
              size="small"
              :min="1"
              :placeholder="t('search_pane.ehentai_limit')" />
            <n-button
              type="primary"
              class="ml-auto"
              size="small"
              :loading="importingEhentaiFavorites"
              @click="importEhentaiFavorites">
              {{ t('search_pane.ehentai_import') }}
            </n-button>
          </div>
        </div>
      </n-collapse-item>
    </n-collapse>

    <div
      v-if="store.searchResult !== undefined"
      ref="comicCardContainerRef"
      class="flex flex-col gap-row-2 overflow-auto box-border px-2">
      <comic-card
        v-for="(comic, index) in store.searchResult.comics"
        :key="comic.id"
        :search="search"
        v-model:comic="store.searchResult.comics[index]" />
    </div>

    <n-pagination
      v-if="store.searchResult !== undefined"
      class="box-border p-2 pt-0 mt-auto"
      :page-count="store.searchResult.totalPage"
      :page="currentPage"
      @update:page="handlePageChange" />
  </div>
</template>
