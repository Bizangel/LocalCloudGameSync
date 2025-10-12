use std::{fs::OpenOptions, io::Write, path::Path};

use globset::GlobSet;
use local_cloud_game_sync::{
    commands::{CheckSyncResult, check_sync_command, pull_command, push_command},
    config::RuntimeSyncConfig,
    tree_utils::tree_folder_hash,
};

use crate::tests_common::{
    temp_folder::TestTempFolder, test_sync_client::test_sync_client_builder::TestSyncClientBuilder,
};

pub struct TestSyncClient {
    pub config: RuntimeSyncConfig,
    _client_folder: TestTempFolder,
}

impl TestSyncClient {
    pub fn builder() -> TestSyncClientBuilder {
        TestSyncClientBuilder::new()
    }

    pub fn check_sync(&self) -> CheckSyncResult {
        check_sync_command(&self.config, false).expect("Unable to check sync status")
    }

    pub fn push(&self) -> Result<(), String> {
        push_command(&self.config, None)
    }

    pub fn pull(&self) -> Result<(), String> {
        pull_command(&self.config, None)
    }

    pub fn get_local_hash(&self) -> String {
        tree_folder_hash(&self.config.local_save_folder, &GlobSet::empty()).unwrap()
    }

    // Helper for simulating game play
    pub fn modify_stored_save(&self) -> Result<(), String> {
        // Modify files in the test folder to simulate gameplay
        // This would be implemented based on your test file setup
        let save_path = self.config.local_save_folder.join("save_state.json");
        if !save_path.exists() {
            return Err(String::from(
                "Cannot modify save path as didn't find save_state.json",
            ));
        }

        let mut filebuf = OpenOptions::new()
            .write(true)
            .append(true)
            .create(false)
            .open(save_path)
            .map_err(|e| format!("{}", e))?;

        filebuf
            .write_all(b"edited_json")
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[path = "./test_sync_client_builder.rs"]
mod test_sync_client_builder;

#[path = "./test_sync_client_assert.rs"]
mod test_sync_client_assert;

pub use test_sync_client_assert::AssertableCheckSyncResult;
