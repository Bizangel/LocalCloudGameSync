// use crate::commands::common::validate_and_process_sync_config;
// use crate::{local_save_config::get_config, remote_lock::RemoteLock, tree_utils};
// use std::path;

pub fn sync_command(save_key: &String) -> Result<(), String> {
    // let (save_folder_path, ignore_globset) = validate_and_process_sync_config(save_key)?;

    // let res = tree_utils::tree_folder_hash(&save_folder_path, &ignore_globset)?;
    // println!("checksum {}", res);

    // copy temp folder
    // let _ = tree_utils::tree_folder_temp_copy(&save_folder_path, &ignore_globset)?;

    // 1. Acquire remote lock.
    // let _lock = RemoteLock::acquire(&config.ssh_host)
    //     .map_err(|e| format!("Unable to get remote lock:\n{}", e))?;

    // if !_lock.is_acquired() {
    //     println!("Unable to obtain remote lock - stopping sync");
    //     return Ok(());
    // }

    // println!("{:#?}", config);

    Ok(())
}
