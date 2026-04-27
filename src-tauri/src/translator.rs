use std::{collections::HashMap, sync::OnceLock};

use anyhow::Context;
use serde::Deserialize;

// Source data: https://github.com/scooderic/exhentai-tags-chinese-translation
const ZH_CN_DICT: &str = include_str!("../resources/ehtags-cn.json");

static TAG_TRANSLATIONS: OnceLock<anyhow::Result<HashMap<String, String>>> = OnceLock::new();

#[derive(Deserialize)]
struct TagTranslationEntry {
    k: String,
    v: String,
}

pub fn translate_tags(tags: &[String], target_language: &str) -> anyhow::Result<Vec<String>> {
    if tags.is_empty() || !target_language.eq_ignore_ascii_case("zh-CN") {
        return Ok(tags.to_vec());
    }

    let translations = translations()?;

    Ok(tags
        .iter()
        .map(|tag| {
            translations
                .get(&normalize_tag(tag))
                .cloned()
                .unwrap_or_else(|| tag.clone())
        })
        .collect())
}

fn translations() -> anyhow::Result<&'static HashMap<String, String>> {
    TAG_TRANSLATIONS
        .get_or_init(load_translations)
        .as_ref()
        .map_err(|err| anyhow::anyhow!(err.to_string()))
}

fn load_translations() -> anyhow::Result<HashMap<String, String>> {
    let entries: Vec<TagTranslationEntry> = serde_json::from_str(ZH_CN_DICT)
        .context("Failed to parse embedded zh-CN tag dictionary")?;

    Ok(entries
        .into_iter()
        .map(|entry| (normalize_tag(&entry.k), entry.v))
        .collect())
}

fn normalize_tag(tag: &str) -> String {
    tag.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::translate_tags;

    #[test]
    fn translates_known_tags_with_embedded_dictionary() {
        let tags = vec!["big breasts".to_string(), "yuri".to_string()];

        let translated = translate_tags(&tags, "zh-CN").unwrap();

        assert_eq!(translated.len(), 2);
        assert_ne!(translated[0], tags[0]);
        assert_ne!(translated[1], tags[1]);
    }

    #[test]
    fn keeps_unknown_tags() {
        let tags = vec!["not-a-real-tag".to_string()];

        let translated = translate_tags(&tags, "zh-CN").unwrap();

        assert_eq!(translated, tags);
    }
}
