use crate::config::RuntimeSyncConfig;
use crate::remote_save_client::remote_lock::RemoteLock;
use crate::remote_save_client::ssh_save_client::SshSaveClient;
use crate::tree_utils::UploadTempFolder;

pub trait RemoteSaveClient<'c> {
    fn init(config: &'c RuntimeSyncConfig) -> Self
    where
        Self: Sized;

    /// Gets the current remote HEAD. This is best - effort so be wary of race-conditions.
    fn get_remote_head(&self) -> Result<Option<String>, String>;

    /// Triggers a remote snapshot process for the current save key configuration.
    /// Should only be triggered if there is something to snapshot. (If HEAD exists)
    fn remote_snapshot(&self) -> Result<(), String>;

    /// Acquires a remote lock.
    /// This should promise that any other clients of the sample implementation will not conflict and modify the remote repo while the lock is held.
    fn acquire_lock<'l>(&'l self) -> Result<impl RemoteLock<'l>, String>;

    /// Pushes to the remote save repository - overwriting the destination and updating the remote HEAD.
    /// This function should implement a mirror functionality - deleting any existing files present in dst but not in src.
    fn push(&self, path: &UploadTempFolder, new_head_hash: &str) -> Result<(), String>;

    /// Pulls from the remote save repository - overwriting the local folder.
    /// Does NOT update local HEAD.
    /// This function should NOT implement a mirror functionality - existing files present in dst should be kept regardless.
    fn pull(&self) -> Result<(), String>;
}

pub fn get_default_remote_save_client<'c>(
    config: &'c RuntimeSyncConfig,
) -> impl RemoteSaveClient<'c> {
    return SshSaveClient::init(config);
}
