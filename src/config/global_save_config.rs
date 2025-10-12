use crate::config::config_commons::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSaveOptionsJson {
    pub ssh_host: String,
    pub ssh_port: Option<u32>,
    pub remote_sync_root: String,
    pub sync_config_folder_path: Option<String>,
    pub local_head_folder: Option<String>,
}

impl GlobalSaveOptionsJson {
    pub fn get_global_config(
        global_config_override: Option<&Path>,
    ) -> Result<Option<GlobalSaveOptionsJson>, String> {
        let filepath = match global_config_override {
            Some(config) if config.exists() => config.to_path_buf(),
            Some(config) => {
                return Err(format!(
                    "Provided config {} does not exist!",
                    config.display()
                ));
            }
            None => get_global_sync_config_path()?,
        };

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
        if parsed.remote_sync_root.is_empty() {
            return Err(format!(
                "remote_sync_root key must not be empty in global config!"
            ));
        }
        if !parsed.remote_sync_root.starts_with("/") {
            return Err(format!("remote_sync_root must be absolute path!"));
        }

        if parsed.remote_sync_root.ends_with("/") {
            return Err(format!("remote_sync_root must not end with /"));
        }

        if parsed
            .sync_config_folder_path
            .as_ref()
            .is_some_and(|x| !Path::new(&x).exists())
        {
            return Err(format!("Provided sync_config_folder_path does not exist!"));
        }

        if parsed
            .local_head_folder
            .as_ref()
            .is_some_and(|x| !Path::new(&x).exists())
        {
            return Err(format!("Provided local_head_folder does not exist!"));
        }

        Ok(Some(parsed))
    }
}
