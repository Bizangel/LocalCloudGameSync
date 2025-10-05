use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::RuntimeSyncConfig;
use crate::remote_save_client::remote_lock::RemoteLock;
use crate::remote_save_client::ssh_save_client::ssh_utils::ssh_command;

pub const LOCK_FOLDER: &str = "/tmp/local_cloud_game_sync.lock";
pub const STALE_TIMEOUT_SECS: u64 = 300; // 5 mins

/// Struct to represent a remote lock, ensuring cleanup
pub struct SshRemoteLock<'c> {
    config: &'c RuntimeSyncConfig,
    acquired: bool,
}

impl<'c> RemoteLock<'c> for SshRemoteLock<'c> {
    /// Attempt to acquire the lock
    fn acquire(config: &'c RuntimeSyncConfig) -> Result<Self, String> {
        // Check if lock exists
        let test_lock_cmd = format!("[ -d {} ]", LOCK_FOLDER);
        let lock_exists = ssh_command(&config.ssh_host, config.ssh_port, &test_lock_cmd)?
            .code
            .success();

        if lock_exists {
            // Read timestamp
            let read_ts_cmd = format!("cat {}/timestamp", LOCK_FOLDER);
            if let Ok(output) = ssh_command(&config.ssh_host, config.ssh_port, &read_ts_cmd) {
                if let Ok(ts_str) = String::from_utf8(output.stdout) {
                    if let Ok(ts) = ts_str.trim().parse::<u64>() {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        let expiry_timestamp = ts + STALE_TIMEOUT_SECS;

                        if expiry_timestamp < now {
                            let rm_cmd = format!("rm -rf {}", LOCK_FOLDER);
                            ssh_command(&config.ssh_host, config.ssh_port, &rm_cmd)?;
                        }
                    }
                }
            }
        }

        // Try to create the lock directory atomically
        let mkdir_cmd = format!("mkdir {}", LOCK_FOLDER);
        let status = ssh_command(&config.ssh_host, config.ssh_port, &mkdir_cmd)?;

        if status.code.success() {
            // Lock acquired, write timestamp inside lock-dir/timestamp
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let write_ts_cmd = format!("echo {} > {}/timestamp", timestamp, LOCK_FOLDER);
            ssh_command(&config.ssh_host, config.ssh_port, &write_ts_cmd)?;

            println!("Remote Lock acquired: {}", LOCK_FOLDER);
            Ok(Self {
                config,
                acquired: true,
            })
        } else {
            // Could not acquire lock
            Ok(Self {
                config,
                acquired: false,
            })
        }
    }

    /// Returns whether we successfully acquired the lock
    fn is_acquired(&self) -> bool {
        self.acquired
    }
}

// Ensure lock cleanup when RemoteLock is dropped
impl<'c> Drop for SshRemoteLock<'c> {
    fn drop(&mut self) {
        if self.acquired {
            let rmdir_cmd = format!("rm {}/timestamp && rmdir {}", LOCK_FOLDER, LOCK_FOLDER);
            match ssh_command(&self.config.ssh_host, self.config.ssh_port, &rmdir_cmd) {
                Ok(status) if status.code.success() => {
                    println!("Remote Lock released: {}", LOCK_FOLDER);
                }
                Ok(status) => {
                    eprintln!("Failed to remove lock, exit code: {:?}", status.code.code());
                }
                Err(e) => {
                    eprintln!("Error releasing lock: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "./ssh_remote_lock_test.rs"]
mod ssh_remote_lock_test;
