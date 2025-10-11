use std::path::Path;

use globset::GlobSet;
use local_cloud_game_sync::{
    commands::{CheckSyncResult, check_sync_command, push_command},
    config::config_commons::REMOTE_SAVES_FOLDER_NAME,
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

    pub fn get_local_hash(&self) -> String {
        tree_folder_hash(&self.local_save_folder.path, &GlobSet::empty()).unwrap()
    }

    pub fn get_remote_hash(&self) -> String {
        tree_folder_hash(
            &Path::new(REMOTE_TEST_SAVE_PATH)
                .join(REMOTE_SAVES_FOLDER_NAME)
                .join(&self.local_config.sync_key),
            &GlobSet::empty(),
        )
        .unwrap()
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

#[path = "./test_sync_client_assert.rs"]
mod test_sync_client_assert;
