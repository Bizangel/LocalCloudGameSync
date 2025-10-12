use crate::{config::default_sync_config_path, utils};

pub fn open_default_config_file() -> Result<(), String> {
    let configspath = default_sync_config_path()?;
    let _ = utils::open_file(&configspath).map_err(|e| e.to_string())?;
    Ok(())
}
