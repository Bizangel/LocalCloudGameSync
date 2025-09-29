use crate::{local_save_config::get_config, remote_lock::RemoteLock};
use std::path;

pub fn sync_command(save_key: &String) -> Result<(), String> {
    let config = get_config(save_key)?;
    let Some(config) = config else {
        println!("Configuration not found for key {}", save_key);
        return Ok(());
    };

    let cfg = path::Path::new(&config.save_folder_path);
    if !cfg.exists() {
        println!(
            "Given save folder path {} does not exist - unable to sync",
            config.save_folder_path
        );
        return Ok(());
    }

    // 1. Acquire remote lock.
    let _lock = RemoteLock::acquire(&config.ssh_host)
        .map_err(|e| format!("Unable to get remote lock:\n{}", e))?;

    println!("{:#?}", config);

    Ok(())
}
