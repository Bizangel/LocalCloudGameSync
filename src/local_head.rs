use std::fs;
use std::path::{Path, PathBuf};

use globset::GlobSet;

use crate::common::Revision;
use crate::config::RuntimeSyncConfig;
use crate::tree_utils::tree_folder_hash;
use crate::utils::get_unix_timestamp_secs;

pub fn get_local_head_filepath(sync_config: &RuntimeSyncConfig) -> PathBuf {
    return sync_config
        .local_head_folder
        .join(format!("{}.HEAD", &sync_config.remote_sync_key));
}

pub fn write_local_head(
    sync_config: &RuntimeSyncConfig,
    new_head: &Revision,
) -> Result<(), String> {
    let local_head_path = get_local_head_filepath(sync_config);

    fs::write(local_head_path, new_head.serialize())
        .map_err(|e| format!("Unable to update local head hash\n{e}"))?;
    Ok(())
}

pub fn read_local_head(sync_config: &RuntimeSyncConfig) -> Result<Option<Revision>, String> {
    let local_head_path = get_local_head_filepath(sync_config);
    if !local_head_path.exists() {
        return Ok(None);
    }

    let folderbytes =
        fs::read(local_head_path).map_err(|e| format!("Unable to read local head hash\n{e}"))?;

    let headstr = String::from_utf8(folderbytes)
        .map_err(|e| format!("Invalid UTF8 bytes reading local head hash\n{e}"))?;

    let rev = Revision::deserialize(&headstr)?;
    Ok(Some(rev))
}

pub fn generate_current_head(path: &Path, ignore_globset: &GlobSet) -> Result<Revision, String> {
    let hash = tree_folder_hash(path, ignore_globset)?;

    return Ok(Revision {
        hash: hash,
        timestamp: get_unix_timestamp_secs(),
    });
}
