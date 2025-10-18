use crate::config::RuntimeSyncConfig;
use crate::local_head::{generate_current_head, write_local_head};
use crate::remote_save_client::{RemoteLock, RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::tree_folder_temp_copy;

pub fn push_command(
    sync_config: &RuntimeSyncConfig,
    push_if_head: Option<&str>,
) -> Result<(), String> {
    push_command_with_update_callback(sync_config, push_if_head, |_| {})
}

pub fn push_command_with_update_callback<F>(
    sync_config: &RuntimeSyncConfig,
    pull_if_head: Option<&str>,
    update_callback: F,
) -> Result<(), String>
where
    F: Fn(String),
{
    let client = get_default_remote_save_client(&sync_config);

    // 1. Get remote lock
    update_callback("Getting Remote Lock...".to_string());
    let _lock = client.acquire_lock()?;
    if !_lock.is_acquired() {
        return Err(String::from(
            "Unable to acquire lock - someone else has it.",
        ));
    }
    update_callback("Remote lock acquired".to_string());

    // 2. Get HEAD contents
    update_callback("Reading Remote HEAD".to_string());
    let remote_head = client.get_remote_head()?;
    // 2.1. Check if head matches as expected - if provided
    if let Some(push_if_head) = pull_if_head {
        let remote_head_hash: String = remote_head.clone().map(|x| x.hash).unwrap_or_default();
        if remote_head_hash != push_if_head {
            return Err(format!(
                "HEAD was modified between check and push. Expected: {push_if_head} Found: {remote_head_hash}. Please try again."
            ));
        }
    };

    // 3. Get current hash - stop if remote already has same hash.
    // NOTE: This does not check or rely on current local uploaded logic - this only relies on existing runtime-based logic.
    // Any decision handling logic should be handled by other commands.
    update_callback("Comparing with local files...".to_string());
    let local_hash = generate_current_head(&sync_config.local_save_folder, &sync_config)?;
    if remote_head.clone().is_some_and(|head| head == local_hash) {
        println!("Remote is up-to-date found same HEAD: {local_hash}");
        return Ok(());
    }

    update_callback("Snapshotting previous save version...".to_string());
    // 4. Perform remote snapshot
    match remote_head.as_ref() {
        Some(head) => {
            println!(
                "Found existing data for {} in remote - Triggering remote Snapshot",
                sync_config.remote_sync_key
            );
            client.remote_snapshot()?;
            println!("Successfully snapshotted remote HEAD: {}", head);
        }
        None => {
            println!("No remote HEAD found - skipping snapshot")
        }
    };

    // 5. Actually push.
    update_callback("Uploading game saves...".to_string());
    let temp_folder =
        tree_folder_temp_copy(&sync_config.local_save_folder, &sync_config.ignore_globset)?;
    client.push(&temp_folder, &local_hash)?;
    println!("Pushed to remote new HEAD {local_hash} successfully!");

    update_callback("Updating local repository file...".to_string());
    // 6. Update local head
    write_local_head(&sync_config, &local_hash)?;
    println!("Successfully updated local head");

    // 7. Perform snapshot again after update.
    update_callback("Snapshotting newly uploaded save version...".to_string());
    println!("Triggering post-upload remote snapshot");
    client.remote_snapshot()?;
    println!("Successfully snapshotted HEAD: {}", local_hash);
    update_callback("Successfully uploaded to remote...".to_string());

    Ok(())
}
