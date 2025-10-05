use std::path::Path;

use crate::config::RuntimeSyncConfig;
use crate::remote_save_client::remote_lock::RemoteLock;
use crate::remote_save_client::ssh_save_client::SshSaveClient;

pub trait RemoteSaveClient<'c> {
    fn init(config: &'c RuntimeSyncConfig) -> Self
    where
        Self: Sized;

    fn get_remote_head(&self) -> Result<Option<String>, String>;
    fn remote_backup(&self) -> Result<(), String>;
    fn acquire_lock<'l>(&'l self) -> Result<impl RemoteLock<'l>, String>;

    fn push(&self, path: &Path) -> Result<(), String>;
}

pub fn get_default_remote_save_client<'c>(
    config: &'c RuntimeSyncConfig,
) -> impl RemoteSaveClient<'c> {
    return SshSaveClient::init(config);
}
