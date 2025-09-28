use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ssh_utils::ssh_command;

pub const LOCK_FOLDER: &str = "/tmp/local_cloud_game_sync.lock";
pub const STALE_TIMEOUT_SECS: u64 = 300; // 5 mins

/// Struct to represent a remote lock, ensuring cleanup
pub struct RemoteLock<'a> {
    host: &'a str,
    acquired: bool,
}

impl<'a> RemoteLock<'a> {
    /// Attempt to acquire the lock
    pub fn acquire(host: &'a str) -> io::Result<Self> {
        // Check if lock exists
        let test_lock_cmd = format!("[ -d {} ]", LOCK_FOLDER);
        let lock_exists = ssh_command(host, &test_lock_cmd)?.code.success();

        if lock_exists {
            // Read timestamp
            let read_ts_cmd = format!("cat {}/timestamp", LOCK_FOLDER);
            if let Ok(output) = ssh_command(host, &read_ts_cmd) {
                if let Ok(ts_str) = String::from_utf8(output.stdout) {
                    if let Ok(ts) = ts_str.trim().parse::<u64>() {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        let expiry_timestamp = ts + STALE_TIMEOUT_SECS;

                        if expiry_timestamp < now {
                            let rm_cmd = format!("rm -rf {}", LOCK_FOLDER);
                            ssh_command(host, &rm_cmd)?;
                        }
                    }
                }
            }
        }

        // Try to create the lock directory atomically
        let mkdir_cmd = format!("mkdir {}", LOCK_FOLDER);
        let status = ssh_command(host, &mkdir_cmd)?;

        if status.code.success() {
            // Lock acquired, write timestamp inside lock-dir/timestamp
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let write_ts_cmd = format!("echo {} > {}/timestamp", timestamp, LOCK_FOLDER);
            ssh_command(host, &write_ts_cmd)?;
            Ok(Self {
                host,
                acquired: true,
            })
        } else {
            // Could not acquire lock
            Ok(Self {
                host,
                acquired: false,
            })
        }
    }

    /// Returns whether we successfully acquired the lock
    pub fn is_acquired(&self) -> bool {
        self.acquired
    }
}

/// Ensure lock cleanup when RemoteLock is dropped
impl<'a> Drop for RemoteLock<'a> {
    fn drop(&mut self) {
        if self.acquired {
            let rmdir_cmd = format!("rm {}/timestamp && rmdir {}", LOCK_FOLDER, LOCK_FOLDER);
            match ssh_command(self.host, &rmdir_cmd) {
                Ok(status) if status.code.success() => {
                    println!("Lock released: {}", LOCK_FOLDER);
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
