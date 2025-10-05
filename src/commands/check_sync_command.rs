use crate::config::load_and_validate_config;
use crate::local_head::write_local_head;
use crate::remote_save_client::{RemoteLock, RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::{tree_folder_hash, tree_folder_temp_copy};

pub fn check_sync_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;
    let client = get_default_remote_save_client(&config);

    todo!();

    Ok(())
}
