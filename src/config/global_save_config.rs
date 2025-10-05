use crate::config::config_commons::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSaveOptionsJson {
    pub ssh_host: String,
    pub ssh_port: Option<u32>,
    pub remote_save_folder_path: String,
}

impl GlobalSaveOptionsJson {
    pub fn get_global_config() -> Result<Option<GlobalSaveOptionsJson>, String> {
        let filepath = get_global_sync_config_path()?;
        if !filepath.exists() {
            return Ok(None);
        }

        // Read file into memory
        let bytes = fs::read(&filepath)
            .map_err(|e| format!("Could not read file {:?}: {}", filepath, e))?;

        // 1. Parse config
        let parsed: GlobalSaveOptionsJson = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Error parsing configuration:\n{}", e))?;
        // 2. Validate Key
        if parsed.ssh_host.is_empty() {
            return Err(format!("sshHost key must not be empty in global config!"));
        }
        if parsed.remote_save_folder_path.is_empty() {
            return Err(format!(
                "remote_save_folder_path key must not be empty in global config!"
            ));
        }
        if !parsed.remote_save_folder_path.starts_with("/") {
            return Err(format!("remote_save_folder_path must be absolute path!"));
        }

        Ok(Some(parsed))
    }
}
