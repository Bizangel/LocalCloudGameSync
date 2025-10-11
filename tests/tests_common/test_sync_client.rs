use std::path::Path;

use globset::GlobSet;
use local_cloud_game_sync::{
    commands::{CheckSyncResult, check_sync_command, push_command},
    tree_utils::tree_folder_hash,
};

use crate::tests_common::{
    common::REMOTE_TEST_SAVE_PATH,
    restic_helper::ResticSnapshotManifest,
    temp_global_config::TempGlobalConfig,
    temp_local_config::TempLocalConfig,
    test_local_folder::TestTempFolder,
    test_sync_client::test_sync_client_builder::TestSyncClientBuilder,
    utils::{get_remote_restic_snapshots, restore_restic_snapshot},
};

pub struct TestSyncClient {
    _global_config: TempGlobalConfig,

    pub local_save_folder: TestTempFolder,
    pub local_config: TempLocalConfig,
}

impl TestSyncClient {
    pub fn builder() -> TestSyncClientBuilder {
        TestSyncClientBuilder::new()
    }

    pub fn check_sync(&self) -> Result<CheckSyncResult, String> {
        check_sync_command(
            &self.local_config.config_key,
            false,
            Some(&self._global_config.override_path),
        )
    }

    pub fn push(&self) -> Result<(), String> {
        push_command(
            &self.local_config.config_key,
            None,
            Some(&self._global_config.override_path),
        )
    }

    pub fn get_snapshots(&self) -> Vec<ResticSnapshotManifest> {
        get_remote_restic_snapshots(&self.local_config.sync_key).expect("Failed to get snapshots")
    }

    fn get_local_hash(&self) -> String {
        tree_folder_hash(&self.local_save_folder.path, &GlobSet::empty()).unwrap()
    }

    fn get_remote_hash(&self) -> String {
        tree_folder_hash(
            &Path::new(REMOTE_TEST_SAVE_PATH)
                .join("GameSaves")
                .join(&self.local_config.sync_key),
            &GlobSet::empty(),
        )
        .unwrap()
    }

    pub fn assert_snapshot_count(&self, expected: usize) {
        let snapshots = self.get_snapshots();
        assert_eq!(snapshots.len(), expected, "Expected {} snapshots", expected);
    }

    pub fn assert_local_matches_remote(&self) {
        let local_hash = self.get_local_hash();
        let remote_hash = self.get_remote_hash();
        assert_eq!(
            local_hash, remote_hash,
            "Local and remote hashes don't match"
        );
    }

    pub fn assert_local_matches_restored_snapshot(&self) {
        let snapshots = self.get_snapshots();
        let restored = restore_restic_snapshot(&self.local_config.sync_key, &snapshots[0].id)
            .expect("Failed to restore snapshot");

        let local_hash = self.get_local_hash();
        let restored_hash = tree_folder_hash(
            &restored
                .path
                .join("GameSaves")
                .join(&self.local_config.sync_key),
            &GlobSet::empty(),
        )
        .expect("Failed to hash restored snapshot");

        assert_eq!(
            local_hash, restored_hash,
            "Local and restored snapshot hashes don't match"
        );
    }

    // // Helper for simulating game play
    // fn simulate_game_session(&self) -> Result<(), String> {
    //     // Modify files in the test folder to simulate gameplay
    //     // This would be implemented based on your test file setup
    //     Ok(())
    // }
}

#[path = "./test_sync_client_builder.rs"]
mod test_sync_client_builder;
