use std::path::Path;

use crate::config::RuntimeSyncConfig;
use crate::config::config_commons::{REMOTE_HEAD_FOLDER, REMOTE_SAVES_FOLDER_NAME};
use crate::remote_save_client::RemoteSaveClient;
use crate::remote_save_client::remote_lock::RemoteLock;
use crate::remote_save_client::ssh_save_client::ssh_remote_lock::SshRemoteLock;
use crate::remote_save_client::ssh_save_client::ssh_utils::{scp_folder, ssh_command};

pub struct SshSaveClient<'c> {
    config: &'c RuntimeSyncConfig,
}

impl<'c> RemoteSaveClient<'c> for SshSaveClient<'c> {
    fn init(config: &'c RuntimeSyncConfig) -> SshSaveClient<'c> {
        return SshSaveClient { config: config };
    }

    fn get_remote_head(&self) -> Result<Option<String>, String> {
        let exists_command = format!(
            "cd {dir} 2>/dev/null || exit 100; \
        [ -r {REMOTE_HEAD_FOLDER}/{key}.HEAD ] && cat {REMOTE_HEAD_FOLDER}/{key}.HEAD && exit 0; \
        [ -e {REMOTE_HEAD_FOLDER}/{key}.HEAD ] && exit 1; \
        exit 2",
            dir = self.config.remote_save_folder_path,
            key = self.config.remote_backup_key
        );

        let res = ssh_command(&self.config.ssh_host, &exists_command)?;
        return match res.code.code() {
            Some(0) => String::from_utf8(res.stdout)
                .map(|x| Some(String::from(x.trim())))
                .map_err(|e| format!("Unable to read file HEAD {}", e)),
            Some(1) => Err(String::from("Remote HEAD file is not readable")),
            Some(2) => Ok(None),
            Some(_) | None => Err(format!(
                "Error ocurred during checking SSH remote HEAD - Exit Code:{}\n{}",
                res.code_display(),
                res.output_lossy()
            )),
        };
    }

    fn acquire_lock<'l>(&'l self) -> Result<impl RemoteLock<'l>, String> {
        return SshRemoteLock::acquire(self.config);
    }

    fn remote_backup(&self) -> Result<(), String> {
        let exists_command = format!(
            "cd {dir} 2>/dev/null || exit 100; \
        [ ! -r {REMOTE_HEAD_FOLDER}/restic_password ] && exit 99; \
        [ ! -d Snapshots/{key} ] && {{ restic init -r Snapshots/{key} -p {REMOTE_HEAD_FOLDER}/restic_password || exit 98; }}; \
        restic -r Snapshots/test-backup/ -p {REMOTE_HEAD_FOLDER}/restic_password backup RemoteSaves/{key}",
            dir = self.config.remote_save_folder_path,
            key = self.config.remote_backup_key
        );

        let res = ssh_command(&self.config.ssh_host, &exists_command)?;

        return match res.code.code() {
            Some(0) => Ok(()),
            Some(99) => Err(format!(
                "{REMOTE_HEAD_FOLDER}/restic_password does not exist or is unreadable!",
            )),
            Some(_) | None => Err(format!(
                "Error ocurred during SSH restic backup calls - Exit Code:{}\n{}",
                res.code_display(),
                res.output_lossy()
            )),
        };
    }

    fn push(&self, src_path: &Path) -> Result<(), String> {
        let scp_result = scp_folder(
            &self.config.ssh_host,
            src_path,
            &format!(
                "{base}/{REMOTE_SAVES_FOLDER_NAME}/{key}",
                base = &self.config.remote_save_folder_path,
                key = &self.config.remote_backup_key
            ),
        )?;

        return match scp_result.code.code() {
            Some(0) => Ok(()),
            Some(_) | None => Err(format!(
                "Error ocurred during SCP - Exit Code:{}\n{}",
                scp_result.code_display(),
                scp_result.output_lossy()
            )),
        };
    }
}
