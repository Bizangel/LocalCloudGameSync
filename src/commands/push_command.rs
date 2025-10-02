use crate::config::load_and_validate_config;
use crate::remote_lock::RemoteLock;
use crate::ssh_utils;
use crate::tree_utils::tree_folder_temp_copy;

pub fn push_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;

    // 1. Get remote lock
    let _lock = RemoteLock::acquire(&config.ssh_host)?;

    // 2. Get HEAD contents
    let _head = ssh_utils::ssh_cat_head(
        &config.ssh_host,
        &config.remote_save_folder_path,
        &config.remote_backup_key,
    );

    // 3. Prepare temporary copy folder.
    let temp_folder = tree_folder_temp_copy(&config.local_save_folder, &config.ignore_globset)?;

    // 4. Actually upload and nuke
    Ok(())
}
