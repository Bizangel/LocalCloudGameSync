use crate::local_save_config::get_sync_configs_path;
use crate::utils;

pub fn open_config_folder_command() -> Result<(), String> {
    let configspath = get_sync_configs_path()?;
    let _ = utils::open_on_explorer(configspath.as_path()).map_err(|e| e.to_string())?;
    Ok(())
}
