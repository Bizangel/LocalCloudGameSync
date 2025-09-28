use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DATA_DIR_NAME: &str = "local_cloud_game_sync";

#[derive(Deserialize, Debug)]
pub struct LocalSaveOptions {
    remoteBackupKey: String,
    saveFolderPath: String,
    saveIgnoreGlob: Vec<String>,
}

fn get_sync_configs_path() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join("sync_configs");
    Ok(configs_path)
}

pub fn init_configs_folder() -> Result<(), String> {
    let configs_path = get_sync_configs_path()?;

    fs::create_dir_all(&configs_path).map_err(|e| {
        format!(
            "Could not create config directory {:?}: {}",
            configs_path, e
        )
    })?;

    Ok(())
}

fn read_config_file(save_key: &str) -> Result<Option<Vec<u8>>, String> {
    let filepath = get_sync_configs_path()?.join(save_key);
    // If file does not exist, just return None
    if !filepath.exists() {
        return Ok(None);
    }

    // Read file into memory
    let bytes =
        fs::read(&filepath).map_err(|e| format!("Could not read file {:?}: {}", filepath, e))?;

    Ok(Some(bytes))
}

pub fn get_config(save_key: &str) -> Result<Option<LocalSaveOptions>, String> {
    return match read_config_file(save_key)? {
        Some(bytes) => serde_json::from_slice(&bytes).map_err(|x| x.to_string()),
        None => Ok(None), // config file not found
    };
}
