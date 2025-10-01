use crate::local_save_config::{GlobalSaveOptions, get_config, get_global_config};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};

pub fn validate_and_process_sync_config(
    save_key: &str,
) -> Result<(GlobalSaveOptions, PathBuf, GlobSet), String> {
    let globalconfig = get_global_config()?;
    let Some(globalconfig) = globalconfig else {
        return Err(format!(
            "Global config not found! Please run init-config command first."
        ));
    };

    let config = get_config(save_key)?;
    let Some(config) = config else {
        return Err(format!("Configuration not found for key {}", save_key));
    };

    let save_folder_path = Path::new(&config.save_folder_path);
    if !save_folder_path.exists() {
        return Err(format!(
            "Given save folder path {} does not exist - unable to sync",
            config.save_folder_path
        ));
    }

    // 2. Hash folder
    let mut builder = GlobSetBuilder::new();
    for pat in config.save_ignore_glob {
        let pattern = Glob::new(&pat).map_err(|e| format!("Invalid glob pattern: {}", e))?;
        builder.add(pattern);
    }
    let ignore_globset = builder
        .build()
        .map_err(|e| format!("Unable to build globset\n{}", e))?;

    return Ok((globalconfig, save_folder_path.to_owned(), ignore_globset));
}
