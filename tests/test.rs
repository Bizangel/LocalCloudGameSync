use std::fs;
use std::path::Path;
use std::path::PathBuf;

use local_cloud_game_sync::commands::push_command;
use local_cloud_game_sync::config::GlobalSaveOptionsJson;
use local_cloud_game_sync::config::LocalSaveOptionsJson;
use local_cloud_game_sync::config::get_sync_configs_folder;

pub fn get_test_global_config() -> GlobalSaveOptionsJson {
    return GlobalSaveOptionsJson {
        ssh_host: "testuser@localhost".to_string(),
        ssh_port: Some(2222),
        remote_save_folder_path: "/home/testuser/testsaves".to_string(),
    };
}

pub struct TempGlobalConfig {
    override_path: PathBuf,
}

impl TempGlobalConfig {
    pub fn get_global_config() -> TempGlobalConfig {
        let path = Path::new("./test_global_config.json");
        fs::write(
            path,
            serde_json::to_string_pretty(&get_test_global_config()).unwrap(),
        )
        .expect("Couldn't write test global config");
        return TempGlobalConfig {
            override_path: path.to_path_buf(),
        };
    }
}

impl Drop for TempGlobalConfig {
    fn drop(&mut self) {
        fs::remove_file(&self.override_path).expect("Unable to clean temp test file config");
    }
}

pub struct TempTestConfig {
    config_key: String,
}

impl TempTestConfig {
    pub fn with_config(cfg: &LocalSaveOptionsJson) -> TempTestConfig {
        let test_config_key = "__temp_test_config";
        fs::write(
            get_sync_configs_folder()
                .unwrap()
                .join(format!("{}.json", test_config_key)),
            serde_json::to_string_pretty(&cfg).expect("Failed to serialize local config"),
        )
        .expect("Couldn't write test local config");

        return TempTestConfig {
            config_key: test_config_key.to_string(),
        };
    }
}

impl Drop for TempTestConfig {
    fn drop(&mut self) {
        fs::remove_file(
            get_sync_configs_folder()
                .unwrap()
                .join("__temp_test_config.json"),
        )
        .expect("Unable to clean temp test file config");
    }
}

pub fn setup_config(global_config: &GlobalSaveOptionsJson, local_config: &LocalSaveOptionsJson) {
    fs::write(
        "./test_global_config.json",
        serde_json::to_string_pretty(&global_config).expect("Failed to serialize global config"),
    )
    .expect("Couldn't write test global config");
    fs::write(
        get_sync_configs_folder()
            .expect("Could not get sync config folder")
            .join("__temp_test_config.json"),
        serde_json::to_string_pretty(&local_config).expect("Failed to serialize local config"),
    )
    .expect("Couldn't write test local config");
}

#[test]
pub fn mytest() {
    let globalcfg = TempGlobalConfig::get_global_config();
    let _cfg = TempTestConfig::with_config(&LocalSaveOptionsJson {
        remote_sync_key: "testKey".to_string(),
        save_folder_path: "temp_local".to_string(),
        save_ignore_glob: [].to_vec(),
    });

    push_command(&_cfg.config_key, None, Some(&globalcfg.override_path)).expect("Failed to push");
}
