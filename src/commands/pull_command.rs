use crate::config::RuntimeSyncConfig;
use crate::local_head::{generate_current_head, write_local_head};
use crate::remote_save_client::{RemoteLock, RemoteSaveClient, get_default_remote_save_client};

pub fn pull_command(
    sync_config: &RuntimeSyncConfig,
    push_if_head: Option<&str>,
) -> Result<(), String> {
    let client = get_default_remote_save_client(&sync_config);

    // 1. Get remote lock
    let _lock = client.acquire_lock()?;
    if !_lock.is_acquired() {
        return Err(String::from(
            "Unable to acquire lock - someone else has it.",
        ));
    }

    // 2. Get HEAD contents
    let remote_head = client.get_remote_head()?;
    let Some(remote_head) = remote_head else {
        return Err(format!(
            "Unable to pull - no remote data found for given key {}",
            sync_config.remote_sync_key
        ));
    };
    // 2.1. Check if head matches as expected - if provided
    if let Some(push_if_head) = push_if_head {
        if remote_head.hash != push_if_head {
            return Err(format!(
                "HEAD was modified between check and pull. Expected: {push_if_head} Found: {remote_head}. Please try again."
            ));
        }
    };

    // 3. Get current hash - stop if local already has same hash
    // NOTE: This does not check or rely on current local uploaded logic - this only relies on existing runtime-based logic.
    // Any decision handling logic should be handled by other commands.
    let local_hash =
        generate_current_head(&sync_config.local_save_folder, &sync_config.ignore_globset)?;
    if remote_head == local_hash {
        println!("Local is up-to-date found same HEAD: {local_hash}");
        // Ensure head is up to date anyways. Maybe we reached that good new state manually
        write_local_head(&sync_config, &remote_head)?;
        return Ok(());
    }

    // 4. Actually pull
    client.pull()?;
    println!("Pulled local to new HEAD {remote_head} successfully!");

    // 5. Update local head
    write_local_head(&sync_config, &remote_head)?;
    println!("Successfully updated local head");

    Ok(())
}
