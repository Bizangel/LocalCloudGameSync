use globset::{Glob, GlobSetBuilder};

use super::*;

impl SyncOptionsJson {
    pub fn validate(self) -> Result<ValidatedSyncOptions, String> {
        // 1. Validate Ssh Host
        if self.ssh_host.is_empty() {
            return Err(format!("sshHost key must not be empty in global config!"));
        }
        // 2. Port is already validated as part of serde - it can only be integer.
        // 3. Validate remote_sync_root
        if self.remote_sync_root.is_empty() {
            return Err(format!(
                "remoteSyncRoot key must not be empty in global config!"
            ));
        }
        if !self.remote_sync_root.starts_with("/") {
            return Err(format!("remoteSyncRoot must be absolute path!"));
        }

        if self.remote_sync_root.ends_with("/") {
            return Err(format!("remoteSyncRoot must not end with /"));
        }

        // 4. Validate local_head_folder
        let default_head = default_local_head_folder_path()?;
        let local_head_folder: PathBuf = self
            .local_head_folder
            .map(|path| Path::new(&path).to_path_buf())
            .unwrap_or(default_head.clone());

        if !local_head_folder.exists() {
            if local_head_folder == default_head {
                return Err(format!(
                    "Local head folder does not exist! Have you executed init-config ?"
                ));
            }
            return Err(format!(
                "Provided localHeadFolder {} does not exist!",
                local_head_folder.display()
            ));
        }

        // 5. Do NOT validate sync entries. Validate sync entries will be validated when runtime config is created.
        // This is intended - so that a misconfigured sync entry from one game does not break others.
        Ok(ValidatedSyncOptions {
            ssh_host: self.ssh_host,
            ssh_port: self.ssh_port.unwrap_or(DEFAULT_SSH_PORT),
            remote_sync_root: self.remote_sync_root,
            local_head_folder: local_head_folder,
            sync_entries: self.sync_entries,
        })
    }
}

fn validate_remote_key(save_key: &str) -> bool {
    for c in save_key.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }
    !save_key.is_empty()
}

impl SyncEntry {
    pub fn validate(&self) -> Result<ValidatedSyncEntry, String> {
        // 1. Validate Sync Key
        if !validate_remote_key(&self.remote_sync_key) {
            return Err(format!(
                "Invalid JSON configuration - remoteSyncKey given \"{}\" - must only contains [A-Za-z0-9_-]",
                self.remote_sync_key
            ));
        }

        // 2. Validate save folder path
        let expanded_save_path_str = &expand_config_placeholders(&self.save_folder_path);
        let expanded_save_path = Path::new(expanded_save_path_str);
        if !expanded_save_path.exists() {
            return Err(format!(
                "Invalid JSON configuration - saveFolderPath given \"{}\" - does not exist! Verify location or Launch game first to create save folder location",
                self.remote_sync_key
            ));
        }

        // 3. Validate globs
        let mut builder = GlobSetBuilder::new();
        for pat in &self.save_ignore_glob {
            let pattern = Glob::new(&pat).map_err(|e| format!("Invalid glob pattern: {}", e))?;
            builder.add(pattern);
        }
        let ignore_globset = builder
            .build()
            .map_err(|e| format!("Unable to build globset\n{}", e))?;

        Ok(ValidatedSyncEntry {
            remote_sync_key: self.remote_sync_key.clone(),
            save_folder_path: expanded_save_path.to_path_buf(),
            save_ignore_glob: ignore_globset,
        })
    }
}
