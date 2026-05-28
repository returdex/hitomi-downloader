import * as i18n from 'vue-i18n'
import { type Config } from './bindings.ts'
import { MessageSchema, SupportedLocales } from './main.ts'

export function useI18n() {
  return i18n.useI18n<{ message: MessageSchema }, SupportedLocales>()
}

export function buildEhentaiCookie(config: Config): string {
  return [
    ['ipb_member_id', config.ehentaiIpbMemberId],
    ['ipb_pass_hash', config.ehentaiIpbPassHash],
    ['igneous', config.ehentaiIgneous],
  ]
    .filter(([, value]) => value.trim() !== '')
    .map(([key, value]) => `${key}=${value.trim()}`)
    .join('; ')
}

export function hasEhentaiCookie(config: Config): boolean {
  return config.ehentaiIpbMemberId.trim() !== '' && config.ehentaiIpbPassHash.trim() !== ''
}

export function formatComicDate(value: string, locale: string): string {
  const trimmed = value.trim()
  if (trimmed === '') {
    return value
  }

  const date = new Date(trimmed)
  if (Number.isNaN(date.getTime())) {
    return value
  }

  return new Intl.DateTimeFormat(locale, { dateStyle: 'medium' }).format(date)
}
