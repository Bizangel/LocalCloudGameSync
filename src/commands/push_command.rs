use globset::{Glob, GlobSetBuilder};

use crate::commands::common::validate_and_process_sync_config;
use std::path;

pub fn push_command(save_key: &String) -> Result<(), String> {
    let (save_folder_path, ignore_globset) = validate_and_process_sync_config(save_key)?;

    Ok(())
}
