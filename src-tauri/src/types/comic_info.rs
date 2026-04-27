use serde::{Deserialize, Serialize};
use specta::Type;
use yaserde::{YaDeserialize, YaSerialize};

use crate::translator;

use super::{Comic, Tag};

/// https://wiki.kavitareader.com/guides/metadata/comics/
#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type, YaSerialize, YaDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfo {
    #[yaserde(rename = "Manga")]
    pub manga: String,
    /// Comic title
    #[yaserde(rename = "Series")]
    pub series: String,
    #[yaserde(rename = "Writer")]
    pub writer: String,
    #[yaserde(rename = "Publisher")]
    pub publisher: String,
    #[yaserde(rename = "Genre")]
    pub genre: String,
    #[yaserde(rename = "Tags")]
    pub tags: String,
    /// Normal chapter number
    #[yaserde(rename = "Number")]
    pub number: Option<String>,
    /// Volume number
    #[yaserde(rename = "Volume")]
    pub volume: Option<String>,
    /// if the value is `Special`, the chapter will be treated as a special issue by Kavita
    #[yaserde(rename = "Format")]
    pub format: Option<String>,
    /// The number of pages in this chapter
    #[yaserde(rename = "PageCount")]
    pub page_count: i64,
    /// Total number of chapters
    /// - `0` => Ongoing
    /// - `Non-zero` and consistent with `Number` or `Volume` => Completed
    /// - `Other non-zero values` => Ended
    #[yaserde(rename = "Count")]
    pub count: i64,
}

impl From<Comic> for ComicInfo {
    fn from(comic: Comic) -> Self {
        ComicInfo {
            manga: "Yes".to_string(),
            series: comic.title,
            writer: comic.artists.join(", "),
            publisher: "Hitomi".to_string(),
            genre: comic.type_field,
            tags: comic
                .tags
                .into_iter()
                .map(|tag| tag.tag)
                .collect::<Vec<String>>()
                .join(", "),
            number: Some("1".to_string()),
            volume: None,
            format: Some("Special".to_string()),
            #[allow(clippy::cast_possible_wrap)]
            page_count: comic.files.len() as i64,
            count: 1,
        }
    }
}

impl ComicInfo {
    pub fn from_comic(ui_locale: &str, mut comic: Comic) -> Self {
        if should_translate_tags(ui_locale) {
            comic.tags = translate_tags(comic.tags, ui_locale);
        }

        ComicInfo::from(comic)
    }
}

fn translate_tags(tags: Vec<Tag>, ui_locale: &str) -> Vec<Tag> {
    let raw_tags = tags.iter().map(|tag| tag.tag.clone()).collect::<Vec<_>>();

    let translated_tags = match translator::translate_tags(&raw_tags, ui_locale) {
        Ok(translated_tags) if translated_tags.len() == tags.len() => translated_tags,
        Ok(_) => return tags,
        Err(err) => {
            tracing::warn!("Failed to translate tags for ComicInfo export: {err:#}");
            return tags;
        }
    };

    tags.into_iter()
        .zip(translated_tags)
        .map(|(mut tag, translated_tag)| {
            tag.tag = translated_tag;
            tag
        })
        .collect()
}

fn should_translate_tags(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("zh-CN")
}
