use local_cloud_game_sync::config::GlobalSaveOptionsJson;

pub const TEST_SYNC_FOLDER_DATA_PATH_1: &str = "./docker_test/test_folders/test1";

pub const REMOTE_CONTAINER_INTERNAL_SAVE_FOLDER_PATH: &str = "/home/testuser/testsaves";
pub const TEST_GLOBAL_CONFIG_PATH: &str = "./test_global_config.json";
pub const LOCAL_TEST_CONFIG_SAVE_KEY: &str = "__temp_test_config";

pub const LOCAL_TEST_PATH: &str = "./temp_local";
pub const REMOTE_TEST_PATH: &str = "./temp_remote";

pub fn get_test_global_config() -> GlobalSaveOptionsJson {
    return GlobalSaveOptionsJson {
        ssh_host: "testuser@localhost".to_string(),
        ssh_port: Some(2222),
        remote_save_folder_path: REMOTE_CONTAINER_INTERNAL_SAVE_FOLDER_PATH.to_string(),
    };
}
