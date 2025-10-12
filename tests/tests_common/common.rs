use local_cloud_game_sync::config::SyncOptionsJson;

pub const TEST_SYNC_FOLDER_DATA_PATH_1: &str = "./docker_test/test_folders/test1";

pub const REMOTE_CONTAINER_INTERNAL_SAVE_FOLDER_PATH: &str = "/home/testuser/testsaves";
pub const TEST_GLOBAL_CONFIG_PATH: &str = "./test_global_config.json";
pub const LOCAL_TEST_CONFIG_SAVE_KEY: &str = "__temp_test_config";

pub const LOCAL_TEST_SAVE_PATH: &str = "./temp_tests/temp_local";
pub const REMOTE_TEST_SAVE_PATH: &str = "./temp_tests/temp_remote";
pub const TEMP_RESTIC_RESTORE_PATH: &str = "./temp_tests/restic_temp_restored";

pub fn get_test_global_config() -> SyncOptionsJson {
    return SyncOptionsJson {
        ssh_host: "testuser@localhost".to_string(),
        ssh_port: Some(2222),
        remote_sync_root: REMOTE_CONTAINER_INTERNAL_SAVE_FOLDER_PATH.to_string(),
    };
}
