use crate::config::global_save_config::SyncOptionsJson;
use crate::utils::get_steam_common;
use crate::utils::get_steam_compatdata;

use std::env;
use std::fs;
use std::path::PathBuf;

pub const REMOTE_SNAPSHOT_FOLDER_NAME: &str = "Snapshots";
pub const REMOTE_SAVES_FOLDER_NAME: &str = "GameSaves";
pub const REMOTE_HEAD_FOLDER_NAME: &str = ".cloudmeta";

pub const DATA_DIR_NAME: &str = "local_cloud_game_sync";
pub const GLOBAL_SYNC_CONFIG_NAME: &str = "global_sync_config.json";

pub fn get_global_sync_config_path() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join(GLOBAL_SYNC_CONFIG_NAME);
    Ok(configs_path)
}

pub fn get_sync_configs_folder() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join("sync_configs");
    Ok(configs_path)
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

pub fn init_configs_folder() -> Result<PathBuf, String> {
    let configs_path = get_sync_configs_folder()?;

    fs::create_dir_all(&configs_path).map_err(|e| {
        format!(
            "Could not create config directory {:?}: {}",
            configs_path, e
        )
    })?;

    // create global config
    let global_sync_config_path = get_global_sync_config_path()?;
    if global_sync_config_path.exists() {
        return Err(format!(
            "Config already exists at {}. Delete file first if you want to re-create",
            global_sync_config_path.display()
        ));
    }

    let placeholder_global_options = SyncOptionsJson {
        ssh_host: String::from(""),
        ssh_port: Some(22),
        remote_sync_root: String::from(""),
        sync_config_folder_path: None,
        local_head_folder: None,
    };
    fs::write(
        global_sync_config_path,
        serde_json::to_string_pretty(&placeholder_global_options)
            .map_err(|e| format!("Error initializing\n{}", e))?,
    )
    .map_err(|e| format!("Unable to initialize global config \n{e}"))?;

    Ok(configs_path)
}
