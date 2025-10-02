use crate::{config::get_sync_configs_folder, utils};

pub fn open_config_folder_command() -> Result<(), String> {
    let configspath = get_sync_configs_folder()?;
    let _ = utils::open_on_explorer(configspath.as_path()).map_err(|e| e.to_string())?;
    Ok(())
}
