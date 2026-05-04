<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useStore } from '../store'
import { computed, ref } from 'vue'
import { path } from '@tauri-apps/api'
import { appDataDir } from '@tauri-apps/api/path'
import { commands } from '../bindings.ts'
import FloatLabelInput from './FloatLabelInput.vue'
import { buildEhentaiCookie, hasEhentaiCookie } from '../utils.ts'
import { useMessage, useNotification } from 'naive-ui'

const { t } = useI18n()

const store = useStore()
const message = useMessage()
const notification = useNotification()

const showing = defineModel<boolean>('showing', { required: true })

const proxyHost = ref<string>(store.config?.proxyHost ?? '')
const dirFmt = ref<string>(store.config?.dirFmt ?? '')
const testingEhentaiConnectivity = ref<boolean>(false)

const disableProxyHostAndPort = computed(() => store.config?.proxyMode !== 'Custom')

async function showConfigInFileManager() {
  const configName = 'config.json'
  const configPath = await path.join(await appDataDir(), configName)
  const result = await commands.showPathInFileManager(configPath)
  if (result.status === 'error') {
    console.error(result.error)
  }
}

async function testEhentaiConnectivity() {
  if (store.config === undefined || testingEhentaiConnectivity.value) {
    return
  }
  if (!hasEhentaiCookie(store.config) || store.config.ehentaiFavoritesUrl.trim() === '') {
    notification.error({
      title: () => t('settings_dialog.ehentai_cookie.test_invalid'),
    })
    return
  }

  testingEhentaiConnectivity.value = true
  const result = await commands.testEhentaiConnectivity(
    buildEhentaiCookie(store.config),
    store.config.ehentaiFavoritesUrl.trim(),
  )
  testingEhentaiConnectivity.value = false

  if (result.status === 'error') {
    notification.error({
      title: () => result.error.err_title,
      description: () => result.error.err_message,
    })
    return
  }

  message.success(() => t('settings_dialog.ehentai_cookie.test_success', { count: result.data.foundCount }))
}
</script>

<template>
  <n-modal v-model:show="showing" v-if="store.config !== undefined">
    <n-dialog :showIcon="false" :title="t('settings_dialog.name')" @close="showing = false">
      <div class="flex flex-col gap-row-2">
        <n-radio-group class="flex gap-2" v-model:value="store.config.downloadFormat">
          <span>{{ t('settings_dialog.download_format') }}</span>
          <n-radio value="Webp">webp</n-radio>
          <n-tooltip placement="top" trigger="hover">
            {{ t('settings_dialog.avif_warning') }}
            <template #trigger>
              <n-radio value="Avif">avif</n-radio>
            </template>
          </n-tooltip>
        </n-radio-group>
        <n-radio-group class="flex gap-2" v-model:value="store.config.proxyMode">
          {{ t('settings_dialog.proxy_mode') }}
          <n-radio value="System">{{ t('settings_dialog.system_proxy') }}</n-radio>
          <n-radio value="NoProxy">{{ t('settings_dialog.no_proxy') }}</n-radio>
          <n-radio value="Custom">{{ t('settings_dialog.custom_proxy') }}</n-radio>
        </n-radio-group>
        <n-input-group>
          <n-input-group-label size="small">http://</n-input-group-label>
          <n-input
            :disabled="disableProxyHostAndPort"
            v-model:value="proxyHost"
            size="small"
            placeholder=""
            @blur="store.config.proxyHost = proxyHost"
            @keydown.enter="store.config.proxyHost = proxyHost" />
          <n-input-group-label size="small">:</n-input-group-label>
          <n-input-number
            :disabled="disableProxyHostAndPort"
            v-model:value="store.config.proxyPort"
            size="small"
            placeholder=""
            :parse="(x: string) => parseInt(x)" />
        </n-input-group>
        <n-collapse>
          <n-collapse-item :title="t('settings_dialog.ehentai_cookie.name')" name="ehentai-cookie">
            <div class="flex flex-col gap-2">
              <FloatLabelInput
                :label="t('settings_dialog.ehentai_cookie.ipb_member_id')"
                size="small"
                v-model:value="store.config.ehentaiIpbMemberId"
                clearable />
              <FloatLabelInput
                :label="t('settings_dialog.ehentai_cookie.ipb_pass_hash')"
                type="password"
                size="small"
                v-model:value="store.config.ehentaiIpbPassHash"
                clearable />
              <FloatLabelInput
                :label="t('settings_dialog.ehentai_cookie.igneous')"
                type="password"
                size="small"
                v-model:value="store.config.ehentaiIgneous"
                clearable />
              <n-button
                class="ml-auto"
                size="small"
                type="primary"
                secondary
                :loading="testingEhentaiConnectivity"
                @click="testEhentaiConnectivity">
                {{ t('settings_dialog.ehentai_cookie.test') }}
              </n-button>
            </div>
          </n-collapse-item>
        </n-collapse>
        <n-tooltip placement="top" trigger="hover" width="580">
          <i18n-t keypath="settings_dialog.directory_format.directory_level_tips" tag="div" scope="global">
            <template v-slot:slash>
              <span class="rounded bg-gray-500 px-1 text-white">/</span>
            </template>
          </i18n-t>
          <div class="font-semibold mt-2">{{ t('settings_dialog.directory_format.available_fields') }}</div>
          <div>
            <div>
              <span class="rounded bg-gray-500 px-1">id</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.id') }}</span>
            </div>
            <div>
              <span class="rounded bg-gray-500 px-1">title</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.title') }}</span>
            </div>
            <div>
              <span class="rounded bg-gray-500 px-1">type</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.type') }}</span>
            </div>
            <div>
              <span class="rounded bg-gray-500 px-1">artists</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.artists') }}</span>
            </div>
            <div>
              <span class="rounded bg-gray-500 px-1">language</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.language') }}</span>
            </div>
            <div>
              <span class="rounded bg-gray-500 px-1">language_localname</span>
              <span class="ml-2">{{ t('settings_dialog.directory_format.language_localname') }}</span>
            </div>
          </div>
          <div class="font-semibold mt-2">{{ t('settings_dialog.directory_format.for_example') }}</div>
          <div class="bg-gray-200 rounded-md p-1 text-black w-fit">
            {type}/{artists}/[{artists}] {title}({id}) - {language}({language_localname})
          </div>
          <div class="font-semibold">{{ t('settings_dialog.directory_format.directory_result') }}</div>
          <div class="flex gap-1 text-black">
            <div class="bg-gray-200 rounded-md px-2 w-fit">doujinshi</div>
            <span class="rounded bg-gray-500 px-1 text-white">/</span>
            <div class="bg-gray-200 rounded-md px-2 w-fit">mameroku</div>
            <span class="rounded bg-gray-500 px-1 text-white">/</span>
            <div class="bg-gray-200 rounded-md px-2 w-fit">[mameroku] Soushi Souai.(2829145) - chinese(中文)</div>
          </div>
          <template #trigger>
            <n-input-group class="box-border">
              <n-input-group-label size="small">{{ t('settings_dialog.directory_format.name') }}</n-input-group-label>
              <n-input
                v-model:value="dirFmt"
                size="small"
                @blur="store.config.dirFmt = dirFmt"
                @keydown.enter="store.config.dirFmt = dirFmt" />
            </n-input-group>
          </template>
        </n-tooltip>
        <n-button class="ml-auto mt-2" size="small" @click="showConfigInFileManager">
          {{ t('settings_dialog.open_config_directory') }}
        </n-button>
      </div>
    </n-dialog>
  </n-modal>
</template>
