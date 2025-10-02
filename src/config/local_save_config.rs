use crate::config::config_commons::*;
use serde::Deserialize;
use std::fs;

fn validate_remote_key(save_key: &str) -> bool {
    for c in save_key.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }
    !save_key.is_empty()
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalSaveOptionsJson {
    pub remote_backup_key: String,
    pub save_folder_path: String,
    pub save_ignore_glob: Vec<String>,
}

impl LocalSaveOptionsJson {
    pub fn get_save_config(save_config_key: &str) -> Result<Option<LocalSaveOptionsJson>, String> {
        let filename = format!("{}.json", save_config_key);
        let filepath = get_sync_configs_folder()?.join(filename);
        // If file does not exist, just return None
        if !filepath.exists() {
            return Ok(None);
        }

        // Read file into memory
        let bytes = fs::read(&filepath)
            .map_err(|e| format!("Could not read file {:?}: {}", filepath, e))?;

        // 1. Parse config
        let parsed: LocalSaveOptionsJson = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Error parsing configuration:\n{}", e))?;
        // 2. Validate Key
        if !validate_remote_key(&parsed.remote_backup_key) {
            return Err(format!(
                "Invalid JSON configuration - remoteBackupKey given \"{}\" - must only contains [A-Za-z0-9_-]",
                parsed.remote_backup_key
            ));
        }
        // 3. Expand placeholders from function
        let modified = LocalSaveOptionsJson {
            save_folder_path: expand_config_placeholders(&parsed.save_folder_path),
            ..parsed.clone() // clone the rest of the fields
        };
        Ok(Some(modified))
    }
}
