use local_cloud_game_sync::{
    config::config_commons::REMOTE_SAVES_FOLDER_NAME, local_head::read_local_head,
};

use crate::tests_common::utils::read_remote_head_test;

use super::*;

impl TestSyncClient {
    pub fn assert_snapshot_count(&self, expected: usize) {
        let snapshots = self.get_snapshots();
        assert_eq!(snapshots.len(), expected, "Expected {} snapshots", expected);
    }

    pub fn assert_local_data_matches_remote_data(&self) {
        let local_hash = self.get_local_hash();
        let remote_hash = self.get_remote_hash();
        assert_eq!(
            local_hash, remote_hash,
            "Local and remote hashes don't match"
        );
    }

    pub fn assert_is_last_snapshot_restorable_and_matches_local_data(&self) {
        let local_hash = self.get_local_hash();
        self.assert_nth_snapshot_matches_hash(
            &local_hash,
            0,
            "Local and restored snapshot hashes don't match",
        );
    }

    pub fn assert_second_last_snapshot_is_restorable_and_matches_hash(&self, hash: &str) {
        self.assert_nth_snapshot_matches_hash(
            hash,
            1,
            "Second last snapshot hash and restored snapshot hashes don't match",
        );
    }

    pub fn assert_nth_snapshot_matches_hash(
        &self,
        expected_hash: &str,
        reverse_chronological_snapshot_idx: usize,
        message: &str,
    ) {
        let snapshots = self.get_snapshots();
        let snapshot_idx = snapshots.len() - reverse_chronological_snapshot_idx - 1;
        assert!(
            reverse_chronological_snapshot_idx < snapshots.len(),
            "Snapshot index out of bounds"
        );

        let restored =
            restore_restic_snapshot(&self.local_config.sync_key, &snapshots[snapshot_idx].id)
                .expect("Failed to restore snapshot");

        let restored_hash = tree_folder_hash(
            &restored
                .path
                .join(REMOTE_SAVES_FOLDER_NAME)
                .join(&self.local_config.sync_key),
            &GlobSet::empty(),
        )
        .expect("Failed to hash restored snapshot");

        assert_eq!(
            expected_hash, restored_hash,
            "Expected hash {} - found restored hash: {}\n{}",
            expected_hash, restored_hash, message
        );
    }

    pub fn assert_local_head_and_remote_head_matches_local_data(&self) {
        let local_hash = self.get_local_hash();

        let local_head = read_local_head(&self.local_config.sync_key)
            .expect("Unable to read local head")
            .expect("Expected non-empty local head");

        let remote_head = read_remote_head_test(&self.local_config.sync_key)
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
}
