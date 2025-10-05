use std::fs;
use std::path::{Path, PathBuf};

use globset::GlobSet;

use crate::common::Revision;
use crate::config::config_commons::DATA_DIR_NAME;
use crate::tree_utils::tree_folder_hash;
use crate::utils::get_unix_timestamp_secs;

pub const LOCAL_UPLOADED_DIR_NAME: &str = "uploaded";

fn get_uploaded_head_folder() -> Result<PathBuf, String> {
    let base_dir = dirs::data_dir().ok_or("Could not determine data directory")?;
    let configs_path = PathBuf::from(base_dir)
        .join(DATA_DIR_NAME)
        .join(LOCAL_UPLOADED_DIR_NAME);
    Ok(configs_path)
}

fn ensure_uploaded_head_folder_exists() -> Result<(), String> {
    let folder = get_uploaded_head_folder()?;
    if !folder.exists() {
        fs::create_dir(folder).map_err(|e| format!("Error creating uploaded folder {e}"))?;
    }

    Ok(())
}

fn get_local_head_filepath(remote_sync_key: &str) -> Result<PathBuf, String> {
    return Ok(get_uploaded_head_folder()?.join(format!("{remote_sync_key}.HEAD")));
}

pub fn write_local_head(remote_sync_key: &str, new_head: &Revision) -> Result<(), String> {
    ensure_uploaded_head_folder_exists()?;
    let local_head_path = get_local_head_filepath(remote_sync_key)?;

    fs::write(local_head_path, new_head.serialize())
        .map_err(|e| format!("Unable to update local head hash\n{e}"))?;
    Ok(())
}

pub fn read_local_head(remote_sync_key: &str) -> Result<Option<Revision>, String> {
    let local_head_path = get_local_head_filepath(remote_sync_key)?;
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
