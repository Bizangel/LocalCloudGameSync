use crate::config::{
    global_save_config::GlobalSaveOptionsJson, local_save_config::LocalSaveOptionsJson,
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};

/// Runtime config which contains all the necessary values for performing sync actions -
/// generated from the global and specific sync config key given.
pub struct RuntimeSyncConfig {
    pub ssh_host: String,
    pub ssh_port: u32,
    pub remote_sync_key: String,

    /// The path to where to store the remote saves. Must be absolute.
    pub remote_save_folder_path: String,
    pub local_save_folder: PathBuf,
    pub ignore_globset: GlobSet,
}

pub fn load_and_validate_config(
    save_config_key: &str,
    global_config_override: Option<&Path>,
) -> Result<RuntimeSyncConfig, String> {
    let globalconfig = GlobalSaveOptionsJson::get_global_config(global_config_override)?;
    let Some(globalconfig) = globalconfig else {
        return Err(format!(
            "Global config not found! Please run init-config command first."
        ));
    };

    let syncconfig = LocalSaveOptionsJson::get_save_config(save_config_key)?;
    let Some(config) = syncconfig else {
        return Err(format!(
            "Configuration not found for key {}",
            save_config_key
        ));
    };

    let save_folder_path = Path::new(&config.save_folder_path);
    if !save_folder_path.exists() {
        return Err(format!(
            "Given save folder path {} does not exist - unable to sync",
            config.save_folder_path
        ));
    }

    let mut builder = GlobSetBuilder::new();
    for pat in config.save_ignore_glob {
        let pattern = Glob::new(&pat).map_err(|e| format!("Invalid glob pattern: {}", e))?;
        builder.add(pattern);
    }
    let ignore_globset = builder
        .build()
        .map_err(|e| format!("Unable to build globset\n{}", e))?;

    return Ok(RuntimeSyncConfig {
        ssh_host: globalconfig.ssh_host,
        ssh_port: globalconfig.ssh_port.unwrap_or(22),
        remote_save_folder_path: globalconfig.remote_save_folder_path,
        remote_sync_key: config.remote_sync_key,
        local_save_folder: save_folder_path.to_owned(),
        ignore_globset: ignore_globset,
    });
}
