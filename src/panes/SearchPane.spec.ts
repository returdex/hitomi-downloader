import { flushPromises, mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { SearchResult } from '../bindings.ts'
import { commands } from '../bindings.ts'
import { useStore } from '../store.ts'
import SearchPane from './SearchPane.vue'

const notificationError = vi.fn()

vi.mock('naive-ui', async (importOriginal) => {
  const actual = await importOriginal<typeof import('naive-ui')>()
  return {
    ...actual,
    NPagination: {
      props: ['page', 'pageCount'],
      emits: ['update:page'],
      template:
        '<button data-test="pagination" @click="$emit(\'update:page\', 2)">{{ page }}</button>',
    },
    useMessage: () => ({ warning: vi.fn() }),
    useNotification: () => ({
      error: notificationError,
      success: vi.fn(),
    }),
  }
})

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}))

vi.mock('../utils.ts', () => ({
  buildEhentaiCookie: vi.fn(),
  hasEhentaiCookie: vi.fn(),
  useI18n: () => ({ t: (key: string) => key }),
}))

const previousResult: SearchResult = {
  comics: [],
  currentPage: 1,
  totalPage: 3,
  ids: [101, 102, 103],
}

const pageTwoResult: SearchResult = {
  comics: [],
  currentPage: 2,
  totalPage: 3,
  ids: previousResult.ids,
}

function mountSearchPane() {
  const pinia = createPinia()
  setActivePinia(pinia)
  const store = useStore()
  store.searchResult = previousResult

  const wrapper = mount(SearchPane, {
    global: {
      plugins: [pinia],
      stubs: {
        FloatLabelInput: true,
      },
    },
  })

  return { store, wrapper }
}

describe('SearchPane pagination', () => {
  beforeEach(() => {
    notificationError.mockClear()
  })

  it('retains the visible page and previous results when loading the next page fails', async () => {
    vi.spyOn(commands, 'getPage').mockResolvedValue({
      status: 'error',
      error: {
        err_title: 'Synthetic page error',
        err_message: 'Synthetic page request failed',
      },
    })
    const { store, wrapper } = mountSearchPane()

    await wrapper.get('[data-test="pagination"]').trigger('click')
    await flushPromises()

    expect(commands.getPage).toHaveBeenCalledWith(previousResult.ids, 2)
    expect(wrapper.get('[data-test="pagination"]').text()).toBe('1')
    expect(store.searchResult).toEqual(previousResult)
    expect(notificationError).toHaveBeenCalledOnce()
  })

  it('publishes the requested page and matching results together after success', async () => {
    vi.spyOn(commands, 'getPage').mockResolvedValue({
      status: 'ok',
      data: pageTwoResult,
    })
    const { store, wrapper } = mountSearchPane()

    await wrapper.get('[data-test="pagination"]').trigger('click')
    await flushPromises()

    expect(commands.getPage).toHaveBeenCalledWith(previousResult.ids, 2)
    expect(wrapper.get('[data-test="pagination"]').text()).toBe('2')
    expect(store.searchResult).toEqual(pageTwoResult)
    expect(notificationError).not.toHaveBeenCalled()
  })
})
