use crate::config::load_and_validate_config;
use crate::local_head::write_local_head;
use crate::remote_save_client::{RemoteLock, RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::{tree_folder_hash, tree_folder_temp_copy};

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

    // 2. Get HEAD contents
    let remote_head = client.get_remote_head()?;

    // 3. Get current hash - stop if remote already has same hash.
    // NOTE: This does not check or rely on current local uploaded logic - this only relies on existing runtime-based logic.
    // Any decision handling logic should be handled by other commands.
    let local_hash = tree_folder_hash(&config.local_save_folder, &config.ignore_globset)?;
    if remote_head.clone().is_some_and(|head| head == local_hash) {
        println!("Remote is up-to-date found same HEAD: {local_hash}");
        return Ok(());
    }

    // 4. Perform remote snapshot
    if remote_head.is_some() {
        println!(
            "Found existing data for {} in remote - Triggering remote Snapshot",
            config.remote_sync_key
        );
        client.remote_snapshot()?;
        println!(
            "Successfully snapshotted remote HEAD: {}",
            remote_head.unwrap_or_default()
        );
    } else {
        println!("No remote HEAD found - skipping snapshot");
    }

    // 4. Actually push.
    let temp_folder = tree_folder_temp_copy(&config.local_save_folder, &config.ignore_globset)?;
    client.push(&temp_folder, &local_hash)?;
    println!("Pushed to remote new HEAD {local_hash} successfully!");

    // 5. Update last uploaded local head
    write_local_head(&config.remote_sync_key, &local_hash)?;
    println!("Successfully updated local head");

    // 6. Perform snapshot again after update.
    println!("Triggering post-upload remote snapshot");
    client.remote_snapshot()?;
    println!("Successfully snapshotted HEAD: {}", local_hash);

    Ok(())
}
