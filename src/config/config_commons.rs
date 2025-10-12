use crate::config::RuntimeSyncConfig;
use crate::config::sync_options::SyncOptionsJson;
use crate::utils::get_steam_common;
use crate::utils::get_steam_compatdata;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub const REMOTE_SNAPSHOT_FOLDER_NAME: &str = "Snapshots";
pub const REMOTE_SAVES_FOLDER_NAME: &str = "GameSaves";
pub const REMOTE_HEAD_FOLDER_NAME: &str = ".cloudmeta";
pub const DATA_DIR_NAME: &str = "local_cloud_game_sync";

// defaults
pub const DEFAULT_SSH_PORT: u32 = 22;
pub const DEFAULT_SYNC_CONFIG_NAME: &str = "sync_config.json";
pub const DEFAULT_HEAD_FOLDER_NAME: &str = "uploaded";

pub fn default_sync_config_path() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join(DEFAULT_SYNC_CONFIG_NAME);
    Ok(configs_path)
}

pub fn default_local_head_folder_path() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let head_folder_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join(DEFAULT_HEAD_FOLDER_NAME);
    Ok(head_folder_path)
}

pub fn expand_config_placeholders(input: &str) -> String {
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

pub fn init_default_config() -> Result<PathBuf, String> {
    let sync_config_path = default_sync_config_path()?;
    let head_folder_path = default_local_head_folder_path()?;
    let data_path = sync_config_path
        .parent()
        .ok_or("Internal error determining config path")?;

    if sync_config_path.exists() {
        return Err(format!(
            "Config already exists at {}. Delete file first if you want to re-create",
            sync_config_path.display()
        ));
    }

    // 1. Init default config.
    fs::create_dir_all(&data_path).map_err(|e| {
        format!(
            "Could not create config directory {:?}: {}",
            sync_config_path, e
        )
    })?;

    let default_global_options = SyncOptionsJson {
        ssh_host: String::from(""),
        ssh_port: Some(22),
        remote_sync_root: String::from(""),
        local_head_folder: None,
        sync_entries: vec![],
    };
    fs::write(
        &sync_config_path,
        serde_json::to_string_pretty(&default_global_options)
            .map_err(|e| format!("Error initializing\n{}", e))?,
    )
    .map_err(|e| format!("Unable to initialize sync config \n{e}"))?;

    // 2. Ensure that head folder is also created.
    fs::create_dir_all(&head_folder_path).map_err(|e| {
        format!(
            "Could not create HEAD folder directory {:?}: {}",
            head_folder_path, e
        )
    })?;

    Ok(sync_config_path)
}

pub fn load_config(
    sync_key: &str,
    config_file_override: Option<&Path>,
) -> Result<RuntimeSyncConfig, String> {
    // 1. Get config path + Ensure file exists
    let default_config_path = default_sync_config_path()?;
    let config_file = config_file_override.unwrap_or(&default_config_path);

    if !config_file.exists() {
        if config_file == default_config_path {
            return Err(format!(
                "Sync config file does not exist! Have you executed init-config ?"
            ));
        }
        return Err(format!(
            "Provided sync config file {} does not exist!",
            config_file.display()
        ));
    }

    // 2. Parse file
    let bytes = fs::read(config_file).map_err(|e| format!("Error reading config file: {}", e))?;
    let parsed_options: SyncOptionsJson =
        serde_json::from_slice(&bytes).map_err(|e| format!("Error parsing config file\n{}", e))?;

    // 3. Validate file and load it.
    let config = RuntimeSyncConfig::validate(parsed_options, sync_key)?;
    Ok(config)
}
