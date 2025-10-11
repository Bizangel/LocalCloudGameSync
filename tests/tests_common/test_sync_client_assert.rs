use local_cloud_game_sync::{
    config::config_commons::REMOTE_SAVES_FOLDER_NAME, local_head::read_local_head,
};

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

    pub fn assert_is_snapshot_restorable_and_matches_local_data(&self) {
        let snapshots = self.get_snapshots();
        let restored = restore_restic_snapshot(&self.local_config.sync_key, &snapshots[0].id)
            .expect("Failed to restore snapshot");

        let local_hash = self.get_local_hash();
        let restored_hash = tree_folder_hash(
            &restored
                .path
                .join(REMOTE_SAVES_FOLDER_NAME)
                .join(&self.local_config.sync_key),
            &GlobSet::empty(),
        )
        .expect("Failed to hash restored snapshot");

        assert_eq!(
            local_hash, restored_hash,
            "Local and restored snapshot hashes don't match"
        );
    }

    // pub fn assert_local_head_and_remote_head_matches_local_data(&self) {
    //     let local_hash = self.get_local_hash();

    //     let local_head =
    //         read_local_head(&self.local_config.sync_key).expect("Unable to read local head");

    //     &Path::new(REMOTE_TEST_SAVE_PATH)
    //             .join(cloud)
    //             .join(&self.local_config.sync_key),

    //     remote_head =
    //     let local_head = assert_eq!(
    //         local_hash, restored_hash,
    //         "Local and restored snapshot hashes don't match"
    //     );
    // }
}
