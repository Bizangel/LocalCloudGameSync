use std::{fs, path::PathBuf};

use super::*;
use crate::tests_common::{
    common::{
        REMOTE_CONTAINER_INTERNAL_ROOT_FOLDER_PATH, TEST_SSH_HOST, TEST_SSH_PORT,
        TEST_SYNC_FOLDER_DATA_PATH_1, TESTING_RESOURCES_ROOT,
    },
    utils::copy_dir_all,
};

pub struct TestSyncClientBuilder {
    client_name: Option<String>,
    sync_key: Option<String>,
    starting_save_folder: Option<Option<PathBuf>>,
}

impl TestSyncClientBuilder {
    pub fn new() -> Self {
        Self {
            client_name: None,
            sync_key: None,
            starting_save_folder: None,
        }
    }

    pub fn with_client_name(mut self, name: impl Into<String>) -> Self {
        self.client_name = Some(name.into());
        self
    }

    pub fn with_sync_key(mut self, key: impl Into<String>) -> Self {
        self.sync_key = Some(key.into());
        self
    }

    pub fn with_empty_test_folder(mut self) -> Self {
        self.starting_save_folder = Some(None);
        self
    }

    pub fn with_local_test_folder1(mut self) -> Self {
        self.starting_save_folder =
            Some(Some(Path::new(TEST_SYNC_FOLDER_DATA_PATH_1).to_path_buf()));
        self
    }

    pub fn build(self) -> TestSyncClient {
        let key = self.sync_key.expect("No sync_key provided for test client");
        let client_name = self
            .client_name
            .expect("No client name provided - provide a differentiator between clients");

        // Create client root folder
        let client_root = Path::new(TESTING_RESOURCES_ROOT).join(&client_name);
        fs::create_dir(&client_root).expect("Unable to create test client root folder");

        // Create head folder
        let head_folder = client_root.join("local_head");
        fs::create_dir(&head_folder).expect("Unable to create test client local head folder");

        // Create actual client local test data.
        let client_save_folder = client_root.join("test_game_save_folder");
        fs::create_dir(&client_save_folder)
            .expect("Unable to create test client local save folder");
        let src_start_save_folder = self
            .starting_save_folder
            .expect("No starting save folder provided for test client.");

        if let Some(src_start_save_folder) = src_start_save_folder {
            copy_dir_all(&src_start_save_folder, &client_save_folder)
                .expect("Unable to copy test save folder for sync client test");
        }

        let cfg = RuntimeSyncConfig {
            game_display_name: "Test Videogame".to_string(),
            client_name: client_name.clone(),
            ssh_host: TEST_SSH_HOST.to_string(),
            ssh_port: TEST_SSH_PORT,
            remote_sync_key: key,
            remote_sync_root: REMOTE_CONTAINER_INTERNAL_ROOT_FOLDER_PATH.to_string(),
            local_head_folder: head_folder,
            local_save_folder: client_save_folder,
            ignore_globset: GlobSet::empty(),
        };

        TestSyncClient {
            config: cfg,
            _client_name: client_name,
            _client_folder: TestTempFolder::from_path(client_root), // will auto cleanup when client is out
        }
    }
}
