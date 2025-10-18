use local_cloud_game_sync::{
    config::config_commons::REMOTE_SAVES_FOLDER_NAME, local_head::read_local_head,
};

use super::*;
use crate::tests_common::test_remote::TestRemote;

pub trait AssertableCheckSyncResult {
    fn assert_up_to_date(&self);
    fn assert_remote_empty(&self);
    fn assert_fast_forward_remote(&self);
    fn assert_fast_forward_local(&self);
    fn assert_conflict(&self);
}

impl AssertableCheckSyncResult for CheckSyncResult {
    fn assert_up_to_date(&self) {
        assert!(matches!(*self, CheckSyncResult::UpToDate));
    }

    fn assert_remote_empty(&self) {
        assert!(matches!(*self, CheckSyncResult::RemoteEmpty));
    }

    fn assert_fast_forward_remote(&self) {
        assert!(matches!(*self, CheckSyncResult::FastForwardRemote));
    }

    fn assert_fast_forward_local(&self) {
        assert!(matches!(*self, CheckSyncResult::FastForwardLocal));
    }

    fn assert_conflict(&self) {
        assert!(matches!(*self, CheckSyncResult::Conflict { .. }));
    }
}

impl TestSyncClient {
    pub fn assert_local_data_matches_remote_data(&self, remote: &TestRemote) {
        let local_hash = self.get_local_hash();
        let remote_hash = remote.get_remote_hash(&self.config.remote_sync_key);
        assert_eq!(
            local_hash, remote_hash,
            "Local and remote hashes don't match"
        );
    }

    pub fn assert_snapshot_count(&self, remote: &TestRemote, expected: usize) {
        let snapshots = remote
            .get_snapshots(&self.config.remote_sync_key)
            .expect("Error reading snapshots from remote");
        assert_eq!(snapshots.len(), expected, "Expected {} snapshots", expected);
    }

    pub fn assert_is_last_snapshot_restorable_and_matches_local_data(&self, remote: &TestRemote) {
        let local_hash = self.get_local_hash();
        self.assert_nth_snapshot_matches_hash(
            &local_hash,
            &remote,
            0,
            "Local and restored snapshot hashes don't match",
        );
    }

    pub fn assert_second_last_snapshot_is_restorable_and_matches_hash(
        &self,
        remote: &TestRemote,
        hash: &str,
    ) {
        self.assert_nth_snapshot_matches_hash(
            hash,
            &remote,
            1,
            "Second last snapshot hash and restored snapshot hashes don't match",
        );
    }

    pub fn assert_nth_snapshot_matches_hash(
        &self,
        expected_hash: &str,
        remote: &TestRemote,
        reverse_chronological_snapshot_idx: usize,
        message: &str,
    ) {
        let snapshots = remote
            .get_snapshots(&self.config.remote_sync_key)
            .expect("Failed to get remote snapshots");
        let snapshot_idx = snapshots.len() - reverse_chronological_snapshot_idx - 1;
        assert!(
            reverse_chronological_snapshot_idx < snapshots.len(),
            "Snapshot index out of bounds"
        );

        let restored = remote
            .restore_restic_snapshot(&self.config.remote_sync_key, &snapshots[snapshot_idx].id)
            .expect("Failed to restore snapshot");

        let (restored_hash, _unix_ts) = tree_folder_hash(
            &restored
                .path
                .join(REMOTE_SAVES_FOLDER_NAME)
                .join(&self.config.remote_sync_key),
            &GlobSet::empty(),
        )
        .expect("Failed to hash restored snapshot");

        assert_eq!(
            expected_hash, restored_hash,
            "Expected hash {} - found restored hash: {}\n{}",
            expected_hash, restored_hash, message
        );
    }

    pub fn assert_local_head_and_remote_head_matches_local_data(&self, remote: &TestRemote) {
        let local_hash = self.get_local_hash();

        let local_head = read_local_head(&self.config)
            .expect("Unable to read local head")
            .expect("Expected non-empty local head");

        let remote_head = remote
            .read_remote_head(&self.config.remote_sync_key)
            .expect("Unable to read remote head")
            .expect("Expected non-empty remote head");

        assert_eq!(
            local_hash, local_head.hash,
            "Local data and local HEAD do not match"
        );

        assert_eq!(
            local_hash, remote_head.hash,
            "Remote HEAD and local data do not match"
        );
    }

    pub fn assert_client_is_remote_author(&self, remote: &TestRemote) {
        let remote_head = remote
            .read_remote_head(&self.config.remote_sync_key)
            .expect("Unable to read remote head")
            .expect("Expected non-empty remote head");

        assert_eq!(self._client_name, remote_head.author);
    }
}
