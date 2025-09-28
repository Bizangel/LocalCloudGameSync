use local_cloud_game_sync::remote_lock::{LOCK_FOLDER, RemoteLock, STALE_TIMEOUT_SECS};
use local_cloud_game_sync::ssh_utils::ssh_command;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_HOST: &str = "arcanzu-miniserver";

    #[test]
    fn test_locks_sequentially() {
        // test sequentially - because they are real host resources and rust by defualt parallelizes tests
        test_lock_acquire_and_fail();
        test_stale_lock();
    }

    fn test_lock_acquire_and_fail() {
        // First lock should succeed
        let lock1 = RemoteLock::acquire(TEST_HOST).expect("Failed to acquire lock 1");
        assert!(lock1.is_acquired(), "First lock could not be acquired");

        // Second lock should fail
        let lock2 = RemoteLock::acquire(TEST_HOST).expect("Failed to attempt lock 2");
        assert!(
            !lock2.is_acquired(),
            "Second Lock was acquired when first lock was already held!"
        );
    }

    fn test_stale_lock() {
        // Create an old timestamp manually to simulate a stale lock
        let old_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (STALE_TIMEOUT_SECS + 1);

        // Create lock folder and timestamp
        let _ = ssh_command(
            TEST_HOST,
            &format!(
                "mkdir -p {} && echo {} > {}/timestamp",
                LOCK_FOLDER, old_ts, LOCK_FOLDER
            ),
        );

        // Lock should detect stale and acquire
        let lock =
            RemoteLock::acquire(TEST_HOST).expect("Failed to acquire lock after stale cleanup");
        assert!(
            lock.is_acquired(),
            "Lock was unable to be acquired after stale cleanup"
        );
    }
}
