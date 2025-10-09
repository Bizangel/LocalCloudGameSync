use std::fs;

use local_cloud_game_sync::config::LocalSaveOptionsJson;
use local_cloud_game_sync::config::get_sync_configs_folder;

use super::common::LOCAL_TEST_CONFIG_SAVE_KEY;

pub struct TempLocalConfig {
    pub config_key: String,
    pub sync_key: String,
}

impl TempLocalConfig {
    pub fn with_config(cfg: &LocalSaveOptionsJson) -> TempLocalConfig {
        fs::write(
            get_sync_configs_folder()
                .unwrap()
                .join(format!("{}.json", LOCAL_TEST_CONFIG_SAVE_KEY)),
            serde_json::to_string_pretty(&cfg).expect("Failed to serialize local config"),
        )
        .expect("Couldn't write test local config");

        return TempLocalConfig {
            config_key: LOCAL_TEST_CONFIG_SAVE_KEY.to_string(),
            sync_key: cfg.remote_sync_key.clone(),
        };
    }
}

impl Drop for TempLocalConfig {
    fn drop(&mut self) {
        fs::remove_file(
            get_sync_configs_folder()
                .unwrap()
                .join(format!("{}.json", self.config_key)),
        )
        .expect("Unable to clean temp test file config");
    }
}
