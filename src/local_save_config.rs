use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::utils::get_steam_common;
use crate::utils::get_steam_compatdata;

const DATA_DIR_NAME: &str = "local_cloud_game_sync";

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // applies to all fields
pub struct LocalSaveOptions {
    remote_backup_key: String,
    save_folder_path: String,
    save_ignore_glob: Vec<String>,
}

fn expand_placeholders(input: &str) -> String {
    let mut result = input.to_string();

    let placeholders = [
        ("{{HOME}}", env::var("HOME").unwrap_or_default()),
        ("{{APPDATA}}", env::var("APPDATA").unwrap_or_default()),
        (
            "{{STEAM_COMMON}}",
            get_steam_common()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
        ),
        (
            "{{STEAM_COMPATDATA}}",
            get_steam_compatdata()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
        ),
    ];

    for (placeholder, value) in placeholders.iter() {
        result = result.replace(placeholder, value);
    }

    result
}

pub fn get_sync_configs_path() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join("sync_configs");
    Ok(configs_path)
}

pub fn init_configs_folder() -> Result<PathBuf, String> {
    let configs_path = get_sync_configs_path()?;

    fs::create_dir_all(&configs_path).map_err(|e| {
        format!(
            "Could not create config directory {:?}: {}",
            configs_path, e
        )
    })?;

    Ok(configs_path)
}

fn read_config_file(save_key: &str) -> Result<Option<Vec<u8>>, String> {
    let filename = format!("{}.json", save_key);
    let filepath = get_sync_configs_path()?.join(filename);
    // If file does not exist, just return None
    if !filepath.exists() {
        return Ok(None);
    }

    // Read file into memory
    let bytes =
        fs::read(&filepath).map_err(|e| format!("Could not read file {:?}: {}", filepath, e))?;

    Ok(Some(bytes))
}

fn validate_key(save_key: &str) -> bool {
    for c in save_key.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }
    !save_key.is_empty()
}

pub fn get_config(save_key: &str) -> Result<Option<LocalSaveOptions>, String> {
    return match read_config_file(save_key)? {
        Some(bytes) => {
            // 1. Parse config
            let parsed: LocalSaveOptions = serde_json::from_slice(&bytes)
                .map_err(|e| format!("Error parsing configuration:\n{}", e))?;
            // 2. Validate Key
            if !validate_key(&parsed.remote_backup_key) {
                return Err(format!(
                    "Invalid JSON configuration - remoteBackupKey given \"{}\" - must only contains [A-Za-z0-9_-]",
                    parsed.remote_backup_key
                ));
            }
            // 3. Expand placeholders from function
            let modified = LocalSaveOptions {
                save_folder_path: expand_placeholders(&parsed.save_folder_path),
                ..parsed.clone() // clone the rest of the fields
            };
            Ok(Some(modified))
        }
        None => Ok(None), // config file not found
    };
}
