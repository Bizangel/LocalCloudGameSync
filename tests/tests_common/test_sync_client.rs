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
    _client_name: String,
}

impl TestSyncClient {
    pub fn builder() -> TestSyncClientBuilder {
        TestSyncClientBuilder::new()
    }

    pub fn check_sync(&self) -> CheckSyncResult {
        println!("--- [{}] Starting Check-Sync ---- ", self._client_name);
        let (res, _remote_head) =
            check_sync_command(&self.config, false).expect("Unable to check sync status");
        println!("--- [{}] Finished Check-Sync ---- ", self._client_name);
        return res;
    }

    pub fn push(&self) -> Result<(), String> {
        println!("--- [{}] Starting push ---- ", self._client_name);
        let res = push_command(&self.config, None)?;
        println!("--- [{}] Finished push ---- ", self._client_name);
        Ok(res)
    }

    pub fn pull(&self) -> Result<(), String> {
        println!("--- [{}] Starting pull ---- ", self._client_name);
        let res = pull_command(&self.config, None)?;
        println!("--- [{}] Finished pull ---- ", self._client_name);
        Ok(res)
    }

    pub fn get_local_hash(&self) -> String {
        tree_folder_hash(&self.config.local_save_folder, &GlobSet::empty()).unwrap()
    }

    // Helper for simulating game play
    pub fn modify_stored_save(&self) -> () {
        // Modify files in the test folder to simulate gameplay
        // This would be implemented based on your test file setup
        let save_path = self.config.local_save_folder.join("save_state.json");
        if !save_path.exists() {
            panic!("Cannot modify save path as didn't find save_state.json");
        }

        let mut filebuf = OpenOptions::new()
            .write(true)
            .append(true)
            .create(false)
            .open(save_path)
            .expect("Cannot modify stored save");

        filebuf
            .write_all(b"edited_json")
            .expect("Cannot modify stored save");
    }
}

#[path = "./test_sync_client_builder.rs"]
mod test_sync_client_builder;

#[path = "./test_sync_client_assert.rs"]
mod test_sync_client_assert;

pub use test_sync_client_assert::AssertableCheckSyncResult;
