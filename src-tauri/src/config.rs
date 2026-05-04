use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

use crate::types::{DownloadFormat, ProxyMode};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub download_dir: PathBuf,
    pub export_dir: PathBuf,
    pub ui_locale: String,
    pub enable_file_logger: bool,
    pub download_format: DownloadFormat,
    pub dir_fmt: String,
    pub proxy_host: String,
    pub proxy_mode: ProxyMode,
    pub proxy_port: u16,
    pub ehentai_ipb_member_id: String,
    pub ehentai_ipb_pass_hash: String,
    pub ehentai_igneous: String,
    pub ehentai_favorites_url: String,
    pub ehentai_favorites_download_dir: PathBuf,
    pub ehentai_favorites_limit: Option<usize>,
    pub ehentai_favorites_auto_check: bool,
    pub ehentai_favorites_known_ids: Vec<i32>,
}

impl Config {
    pub fn new(app: &AppHandle) -> anyhow::Result<Config> {
        let app_data_dir = app.path().app_data_dir()?;
        let config_path = app_data_dir.join("config.json");

        let config = if config_path.exists() {
            let config_string = std::fs::read_to_string(config_path)?;
            match serde_json::from_str(&config_string) {
                // if deserialization succeeds, use the deserialized Config
                Ok(config) => config,
                // Otherwise, merge the default configuration with the configuration already in the file
                // to avoid resetting all configuration items when new configuration items are added in the new version
                // after the user upgrades to the new version
                Err(_) => Config::merge_config(&config_string, &app_data_dir),
            }
        } else {
            Config::default(&app_data_dir)
        };
        config.save(app)?;
        Ok(config)
    }

    pub fn save(&self, app: &AppHandle) -> anyhow::Result<()> {
        let app_data_dir = app.path().app_data_dir()?;
        let config_path = app_data_dir.join("config.json");
        let config_string = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_string)?;
        Ok(())
    }

    fn merge_config(config_string: &str, app_data_dir: &Path) -> Config {
        let Ok(mut json_value) = serde_json::from_str::<serde_json::Value>(config_string) else {
            return Config::default(app_data_dir);
        };
        let serde_json::Value::Object(ref mut map) = json_value else {
            return Config::default(app_data_dir);
        };
        let Ok(default_config_value) = serde_json::to_value(Config::default(app_data_dir)) else {
            return Config::default(app_data_dir);
        };
        let serde_json::Value::Object(default_map) = default_config_value else {
            return Config::default(app_data_dir);
        };
        for (key, value) in default_map {
            map.entry(key).or_insert(value);
        }
        Config::migrate_ehentai_cookie_fields(map);
        let Ok(config) = serde_json::from_value(json_value) else {
            return Config::default(app_data_dir);
        };
        config
    }

    fn migrate_ehentai_cookie_fields(map: &mut serde_json::Map<String, serde_json::Value>) {
        let Some(serde_json::Value::String(cookie)) = map.get("ehentaiCookie") else {
            return;
        };
        let cookie = cookie.clone();
        for part in cookie.split(';') {
            let Some((key, value)) = part.trim().split_once('=') else {
                continue;
            };
            let config_key = match key.trim() {
                "ipb_member_id" => "ehentaiIpbMemberId",
                "ipb_pass_hash" => "ehentaiIpbPassHash",
                "igneous" => "ehentaiIgneous",
                _ => continue,
            };
            if map
                .get(config_key)
                .and_then(serde_json::Value::as_str)
                .is_some_and(|existing| !existing.is_empty())
            {
                continue;
            }
            map.insert(
                config_key.to_string(),
                serde_json::Value::String(value.trim().to_string()),
            );
        }
    }

    fn default(app_data_dir: &Path) -> Config {
        Config {
            download_dir: app_data_dir.join("download"),
            export_dir: app_data_dir.join("export"),
            ui_locale: String::new(),
            enable_file_logger: true,
            download_format: DownloadFormat::Webp,
            dir_fmt: "{title} - {id}".to_string(),
            proxy_mode: ProxyMode::System,
            proxy_host: "127.0.0.1".to_string(),
            proxy_port: 7890,
            ehentai_ipb_member_id: String::new(),
            ehentai_ipb_pass_hash: String::new(),
            ehentai_igneous: String::new(),
            ehentai_favorites_url: "https://e-hentai.org/favorites.php".to_string(),
            ehentai_favorites_download_dir: app_data_dir.join("download"),
            ehentai_favorites_limit: Some(25),
            ehentai_favorites_auto_check: false,
            ehentai_favorites_known_ids: Vec::new(),
        }
    }
}
