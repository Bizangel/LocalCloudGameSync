use crate::utils::get_unix_timestamp_secs;

use super::*;
use globset::GlobSet;
use serial_test::serial;
use std::path::Path;

const TEST_SSH_HOST: &str = "testuser@localhost";
const TEST_SSH_PORT: u32 = 2222;

fn get_test_config() -> RuntimeSyncConfig {
    return RuntimeSyncConfig {
        ssh_host: TEST_SSH_HOST.to_string(),
        ssh_port: TEST_SSH_PORT,
        remote_sync_key: "test-key".to_string(),
        remote_sync_root: "/tmp/test-save".to_string(),
        local_save_folder: Path::new("").to_path_buf(),
        ignore_globset: GlobSet::empty(),
    };
}

#[test]
#[serial]
fn test_lock_acquire_and_fail() {
    // First lock should succeed
    let cfg = get_test_config();
    let lock1 = SshRemoteLock::acquire(&cfg).expect("Failed to acquire lock 1");
    assert!(lock1.is_acquired(), "First lock could not be acquired");

    // Second lock should fail
    let lock2 = SshRemoteLock::acquire(&cfg).expect("Failed to attempt lock 2");
    assert!(
        !lock2.is_acquired(),
        "Second Lock was acquired when first lock was already held!"
    );
}

#[test]
#[serial]
fn test_stale_lock() {
    // Create an old timestamp manually to simulate a stale lock
    let old_ts = get_unix_timestamp_secs() - (STALE_TIMEOUT_SECS + 1);

    // Create lock folder and timestamp
    let _ = ssh_command(
        TEST_SSH_HOST,
        TEST_SSH_PORT,
        &format!(
            "mkdir -p {} && echo {} > {}/timestamp",
            LOCK_FOLDER, old_ts, LOCK_FOLDER
        ),
    );

    // Lock should detect stale and acquire
    let cfg = get_test_config();
    let lock = SshRemoteLock::acquire(&cfg).expect("Failed to acquire lock after stale cleanup");
    assert!(
        lock.is_acquired(),
        "Lock was unable to be acquired after stale cleanup"
    );
}
