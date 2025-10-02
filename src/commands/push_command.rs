use crate::config::load_and_validate_config;
use crate::remote_lock::RemoteLock;
use crate::ssh_utils::{self, scp_folder};
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
    // Perform snapshot of existing save remotely

    // 5. Actually push.
    let scp_result = scp_folder(
        &config.ssh_host,
        &temp_folder.path,
        &format!(
            "{base}/{key}",
            base = &config.remote_save_folder_path,
            key = config.remote_backup_key
        ),
    )?;

    scp_result.print();

    // 6. Perform snapshot again after update.

    // Note: We work under the assumption that everything is snapshotted - last upload should've snapshotted the previous save.
    // If backup failed last time - and then you try re-uploading.

    Ok(())
}
