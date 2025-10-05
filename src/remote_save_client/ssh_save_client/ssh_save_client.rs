use crate::common::Revision;
use crate::config::RuntimeSyncConfig;
use crate::config::config_commons::{REMOTE_HEAD_FOLDER_NAME, REMOTE_SAVES_FOLDER_NAME};
use crate::remote_save_client::RemoteSaveClient;
use crate::remote_save_client::remote_lock::RemoteLock;
use crate::remote_save_client::ssh_save_client::ssh_remote_lock::SshRemoteLock;
use crate::remote_save_client::ssh_save_client::ssh_utils::{
    scp_from_remote, scp_to_remote, ssh_command,
};
use crate::tree_utils::UploadTempFolder;

pub struct SshSaveClient<'c> {
    config: &'c RuntimeSyncConfig,
}

impl<'c> RemoteSaveClient<'c> for SshSaveClient<'c> {
    fn init(config: &'c RuntimeSyncConfig) -> SshSaveClient<'c> {
        return SshSaveClient { config: config };
    }

    fn get_remote_head(&self) -> Result<Option<Revision>, String> {
        let exists_command = format!(
            "cd {dir} 2>/dev/null || exit 100; \
        [ -r {REMOTE_HEAD_FOLDER_NAME}/{key}.HEAD ] && cat {REMOTE_HEAD_FOLDER_NAME}/{key}.HEAD && exit 0; \
        [ -e {REMOTE_HEAD_FOLDER_NAME}/{key}.HEAD ] && exit 1; \
        exit 2",
            dir = self.config.remote_save_folder_path,
            key = self.config.remote_sync_key
        );

        let res = ssh_command(&self.config.ssh_host, self.config.ssh_port, &exists_command)?;
        return match res.code.code() {
            Some(0) => {
                let filestr = String::from_utf8(res.stdout)
                    .map_err(|e| format!("Unable to read file HEAD {}", e))?;
                let rev = Revision::deserialize(filestr.trim())?;
                return Ok(Some(rev));
            }
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

    fn remote_snapshot(&self) -> Result<(), String> {
        let exists_command = format!(
            "cd {dir} 2>/dev/null || exit 100; \
        [ ! -r {REMOTE_HEAD_FOLDER_NAME}/restic_password ] && exit 99; \
        [ ! -d Snapshots/{key} ] && {{ restic init -r Snapshots/{key} -p {REMOTE_HEAD_FOLDER_NAME}/restic_password || exit 98; }}; \
        restic -r Snapshots/{key}/ -p {REMOTE_HEAD_FOLDER_NAME}/restic_password backup {REMOTE_SAVES_FOLDER_NAME}/{key}",
            dir = self.config.remote_save_folder_path,
            key = self.config.remote_sync_key
        );

        let res = ssh_command(&self.config.ssh_host, self.config.ssh_port, &exists_command)?;

        return match res.code.code() {
            Some(0) => Ok(()),
            Some(99) => Err(format!(
                "{REMOTE_HEAD_FOLDER_NAME}/restic_password does not exist or is unreadable!",
            )),
            Some(_) | None => Err(format!(
                "Error ocurred during SSH restic backup calls - Exit Code:{}\n{}",
                res.code_display(),
                res.output_lossy()
            )),
        };
    }

    fn push(&self, src_path: &UploadTempFolder, new_head: &Revision) -> Result<(), String> {
        let rmrf_cmd = ssh_command(
            &self.config.ssh_host,
            self.config.ssh_port,
            &format!(
                "rm -rf {base}/{REMOTE_SAVES_FOLDER_NAME}/{key}",
                base = &self.config.remote_save_folder_path,
                key = &self.config.remote_sync_key
            ),
        )?;

        if !rmrf_cmd.code.success() {
            return Err(format!(
                "Error ocurred during pre-SCP cleanup - Exit Code:{}\n{}",
                rmrf_cmd.code_display(),
                rmrf_cmd.output_lossy()
            ));
        }

        let scp_result = scp_to_remote(
            &self.config.ssh_host,
            self.config.ssh_port,
            &src_path.path,
            &format!(
                "{base}/{REMOTE_SAVES_FOLDER_NAME}/{key}",
                base = &self.config.remote_save_folder_path,
                key = &self.config.remote_sync_key
            ),
        )?;

        if !scp_result.code.success() {
            return Err(format!(
                "Error ocurred during during SCP - Exit Code:{}\n{}",
                rmrf_cmd.code_display(),
                rmrf_cmd.output_lossy()
            ));
        }

        let updatehead_cmd = ssh_command(
            &self.config.ssh_host,
            self.config.ssh_port,
            &format!(
                "echo \"{headstr}\" > {base}/{REMOTE_HEAD_FOLDER_NAME}/{key}.HEAD",
                headstr = new_head.serialize(),
                base = &self.config.remote_save_folder_path,
                key = &self.config.remote_sync_key
            ),
        )?;

        if !updatehead_cmd.code.success() {
            return Err(format!(
                "Error updating remote HEAD - Exit Code:{}\n{}",
                rmrf_cmd.code_display(),
                rmrf_cmd.output_lossy()
            ));
        }

        Ok(())
    }

    fn pull(&self) -> Result<(), String> {
        let scp_result = scp_from_remote(
            &self.config.ssh_host,
            self.config.ssh_port,
            &format!(
                "{base}/{REMOTE_SAVES_FOLDER_NAME}/{key}/.", // use this syntax to ensure full copy
                base = &self.config.remote_save_folder_path,
                key = &self.config.remote_sync_key
            ),
            &self.config.local_save_folder,
        )?;

        if !scp_result.code.success() {
            return Err(format!(
                "Error ocurred during during SCP - Exit Code:{}\n{}",
                scp_result.code_display(),
                scp_result.output_lossy()
            ));
        }

        Ok(())
    }
}
