use super::*;
use crate::tests_common::reset_remote::reset_remote_repository;
use local_cloud_game_sync::config::LocalSaveOptionsJson;

pub struct TestSyncClientBuilder {
    empty_remote: bool,
    sync_key: Option<String>,
    local_save_folder: Option<TestTempFolder>,
    ignore_globs: Vec<String>,
}

impl TestSyncClientBuilder {
    pub fn new() -> Self {
        Self {
            empty_remote: false,
            sync_key: None,
            local_save_folder: None,
            ignore_globs: vec![],
        }
    }

    pub fn with_empty_remote(mut self) -> Self {
        self.empty_remote = true;
        self
    }

    pub fn with_sync_key(mut self, key: impl Into<String>) -> Self {
        let keystring: String = key.into();
        assert!(
            keystring.starts_with("__"),
            "test key must begin with __ to avoid conflicts with real user configs!"
        );
        self.sync_key = Some(keystring);
        self
    }

    pub fn with_local_test_folder1(mut self) -> Self {
        self.local_save_folder = Some(TestTempFolder::with_test_local_folder1());
        self
    }

    // pub fn with_ignore_globs(mut self, globs: Vec<String>) -> Self {
    //     self.ignore_globs = globs;
    //     self
    // }

    pub fn build(self) -> TestSyncClient {
        let sync_key = self.sync_key.expect("sync_key is required");
        let globalcfg = TempGlobalConfig::get_global_config();
        let local_save_folder = self.local_save_folder.expect("save folder is required");
        let cfg = TempLocalConfig::from_config(LocalSaveOptionsJson {
            remote_sync_key: sync_key.clone(),
            save_folder_path: local_save_folder.path.to_str().unwrap().to_string(),
            save_ignore_glob: self.ignore_globs,
        });

        // Build side effects
        if self.empty_remote {
            reset_remote_repository();
        }

        TestSyncClient {
            local_save_folder,
            local_config: cfg,
            _global_config: globalcfg,
        }
    }
}
