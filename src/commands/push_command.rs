use crate::config::load_and_validate_config;
use crate::remote_save_client::{RemoteLock, RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::tree_folder_temp_copy;

pub fn push_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;
    let client = get_default_remote_save_client(&config);

    // 1. Get remote lock
    let _lock = client.acquire_lock()?;
    if !_lock.is_acquired() {
        return Err(String::from(
            "Unable to acquire lock - someone else has it.",
        ));
    }

    // // 2. Get HEAD contents
    let _head = client.get_remote_head()?;

    // // 4. Perform remote backup
    // // Perform snapshot of existing save remotely
    // client.remote_backup();

    // if _head.is_some() {
    //     // If head exists - save exists.
    //     ssh_restic_backup(
    //         &config.ssh_host,
    //         &config.remote_save_folder_path,
    //         &config.remote_backup_key,
    //     )?;
    // } else {
    //     println!("No remote save found - skipping backup");
    // }

    // // 5. Actually push.
    let temp_folder = tree_folder_temp_copy(&config.local_save_folder, &config.ignore_globset)?;
    client.push(&temp_folder.path)?;

    // scp_result.print();
    // Update remote head

    // Update last uploaded local head

    // 6. Perform snapshot again after update.

    // Note: We work under the assumption that everything is snapshotted - last upload should've snapshotted the previous save.
    // If backup failed last time - and then you try re-uploading.

    Ok(())
}
