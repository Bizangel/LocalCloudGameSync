use crate::config::sync_options::SyncOptionsJson;
use globset::GlobSet;
use std::path::PathBuf;

/// Runtime config which contains all the necessary values for performing sync actions -
/// generated from the global and specific sync config key given.
pub struct RuntimeSyncConfig {
    // Display names
    pub client_name: String,
    pub game_display_name: String,

    pub ssh_host: String,
    pub ssh_port: u32,
    pub remote_sync_key: String,
    pub local_head_folder: PathBuf,

    /// The path to where to store the remote saves. Must be absolute.
    pub remote_sync_root: String,
    pub local_save_folder: PathBuf,
    pub ignore_globset: GlobSet,
}

impl RuntimeSyncConfig {
    pub fn validate(options: SyncOptionsJson, sync_key: &str) -> Result<RuntimeSyncConfig, String> {
        let validated_options = options.validate()?;

        let sync_entry = validated_options
            .sync_entries
            .iter()
            .find(|x| x.remote_sync_key == sync_key)
            .ok_or(format!(
                "Unable to find sync_key: {} in sync entries in options",
                sync_key
            ))?;

        let validated_sync_entry = sync_entry.validate()?;

        return Ok(RuntimeSyncConfig {
            client_name: validated_options.client_name,
            ssh_host: validated_options.ssh_host,
            ssh_port: validated_options.ssh_port,
            remote_sync_root: validated_options.remote_sync_root,
            local_head_folder: validated_options.local_head_folder,
            // from entry
            remote_sync_key: validated_sync_entry.remote_sync_key,
            local_save_folder: validated_sync_entry.save_folder_path,
            ignore_globset: validated_sync_entry.save_ignore_glob,
            game_display_name: validated_sync_entry.display_name,
        });
    }
}
